use crate::connectors::db::connection::DBConnection;
use crate::connectors::db::reset::{
    create_request, delete_request, get_request_by_id, replace_request, request_exists,
};
use crate::connectors::db::user::get_user_from_email;
use crate::connectors::db::user::{does_email_exist, update_user_by_id};
use crate::connectors::smtp::smtp_service::SMTPService;
use crate::helpers::funcs::is_expired;
use crate::helpers::random::make_random_string;
use crate::helpers::state::get_connection;
use crate::models::reset::InsertRequest;
use crate::models::users::{GetUser, UpdateUser};
use crate::types::error::{Error, RequestErrors};
use crate::types::response::Response;
use crate::types::state::TideState;
use bcrypt::hash;
use bcrypt::verify;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;
use tide::Request;
use validator::Validate;

use super::{init_session, PASSWORD_COST};

// less important less cost
// consts
const COST: u32 = 8;
const HASH_LEN: usize = 32;
const RESET_PASSWORD_SUBJECT: Lazy<String> = Lazy::new(|| "Saladify Password Reset".to_string());
const RESET_PASSWORD_BODY: Lazy<String> = Lazy::new(|| "Your Verification Code is:".to_string());
// 5 minutes
const RESET_DURATION: chrono::TimeDelta = chrono::Duration::minutes(5);
// testing
// const RESET_DURATION: chrono::TimeDelta = chrono::Duration::seconds(5);

// params to reset password
#[derive(Debug, Deserialize, Validate)]
struct ResetPasswordParams {
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 to 50 characters"
    ))]
    password: String,
    code: String,
    #[validate(email(message = "Email is incorrect"))]
    email: String,
}

#[derive(Debug, Deserialize, Validate)]
struct CheckPasswordCodeParams {
    code: String,
    #[validate(email(message = "Email is incorrect"))]
    email: String,
}

// email params
#[derive(Debug, Deserialize, Validate)]
pub struct GetEmailParams {
    #[validate(email(message = "Email is incorrect"))]
    pub email: String,
}

// the post route for reset-password
// email is sent to route
// a request obj is created in the database which stores information related to this password reset request
// if the request object already exists then replace it
pub async fn get_email(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from email

    // get state
    let state = req.state();

    // get connection pool
    let mut conn = get_connection(&mut req);

    // get payload
    let get_email_params: GetEmailParams = match req.body_json().await {
        Ok(params) => params,
        Err(_e) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // validate
    match get_email_params.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    match does_email_exist(&mut conn, get_email_params.email.clone()).await {
        Ok(exists) => {
            if !exists {
                return Error::NotFoundError("Email".to_string()).into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    let user: GetUser = match get_user_from_email(&mut conn, get_email_params.email.clone()).await {
        Ok(user) => user,
        Err(e) => return e.into_response(),
    };

    // make random code
    let code = make_random_string(HASH_LEN);
    // hash random code
    let hashed_code = match hash(&code, COST) {
        Ok(new_code) => new_code,
        Err(e) => return Error::HashError(e).into_response(),
    };

    // get smtp service
    let email_service: Box<&(dyn SMTPService + Sync)> = Box::new(&req.state().email_service);
    //  send email
    // in the future could build html using handlebar or something
    match (*email_service).send_email(
        user.email,
        (*RESET_PASSWORD_SUBJECT).to_owned(),
        (*RESET_PASSWORD_BODY).to_owned() + &code,
    ) {
        Ok(_) => {}
        Err(e) => return e.into_response(),
    };

    // check if reset request already exists
    let already_exists: bool = match request_exists(&mut conn, user.id).await {
        Ok(is_true) => is_true,
        Err(e) => return e.into_response(),
    };

    if already_exists {
        // replace
        let new_request = InsertRequest {
            user_id: user.id,
            code: hashed_code,
            created_at: chrono::Local::now().naive_local(),
        };
        match replace_request(&mut conn, user.id, new_request).await {
            Ok(_) => {}
            Err(e) => return e.into_response(),
        };
    } else {
        // make new one
        let new_request = InsertRequest {
            user_id: user.id,
            code: hashed_code,
            created_at: chrono::Local::now().naive_local(),
        };
        match create_request(&mut conn, new_request).await {
            Ok(_) => {}
            Err(e) => return e.into_response(),
        }
    }
    return Response::empty().into_response();
}

// post request
// it checks if the code is correct
pub async fn check_password_code(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get payload
    let code_params: CheckPasswordCodeParams = match req.body_json().await {
        Ok(params) => params,
        Err(_e) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // validate
    match code_params.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let state = req.state();
    // get connection pool
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    match does_email_exist(&mut conn, code_params.email.clone()).await {
        Ok(exists) => {
            if !exists {
                return Error::NotFoundError("Email".to_string()).into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    let user: GetUser = match get_user_from_email(&mut conn, code_params.email.clone()).await {
        Ok(user) => user,
        Err(e) => return e.into_response(),
    };

    // check if the code exists
    match request_exists(&mut conn, user.id).await {
        Ok(exists) => {
            if !exists {
                return Error::NoPasswordResetError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    // get the code
    let request = match get_request_by_id(&mut conn, user.id).await {
        Ok(request) => request,
        Err(e) => return e.into_response(),
    };

    // check if expired
    match is_expired(request.created_at, RESET_DURATION.clone()) {
        Ok(password_expired) => {
            if password_expired {
                return Error::PasswordResetCodeExpiredError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    match verify(code_params.code, &request.code) {
        Ok(is_correct) => {
            if !is_correct {
                return Error::WrongPasswordResetCodeError().into_response();
            }
        }
        Err(e) => return Error::HashError(e).into_response(),
    }

    return Response::empty().into_response();
}

// that uses the reset password
// uses the code and new password
pub async fn reset_password(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get payload
    let reset_params: ResetPasswordParams = match req.body_json().await {
        Ok(params) => params,
        Err(_e) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // validate
    match reset_params.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let state = req.state();

    // get connection pool
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    match does_email_exist(&mut conn, reset_params.email.clone()).await {
        Ok(exists) => {
            if !exists {
                return Error::NotFoundError("Email".to_string()).into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    let user: GetUser = match get_user_from_email(&mut conn, reset_params.email.clone()).await {
        Ok(user) => user,
        Err(e) => return e.into_response(),
    };

    // check if the code exists
    match request_exists(&mut conn, user.id).await {
        Ok(exists) => {
            if !exists {
                return Error::NoPasswordResetError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }
    // get the code
    let request = match get_request_by_id(&mut conn, user.id).await {
        Ok(request) => request,
        Err(e) => return e.into_response(),
    };

    // check if expired
    match is_expired(request.created_at, RESET_DURATION.clone()) {
        Ok(password_expired) => {
            if password_expired {
                return Error::PasswordResetCodeExpiredError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    // if code  doesnt match
    match verify(reset_params.code, &request.code) {
        Ok(is_correct) => {
            if !is_correct {
                return Error::WrongPasswordResetCodeError().into_response();
            }
        }
        Err(e) => return Error::HashError(e).into_response(),
    }

    let hashed_password = match hash(reset_params.password, PASSWORD_COST) {
        Ok(p) => p,
        Err(e) => return Error::HashError(e).into_response(),
    };

    // update user
    let update_user = UpdateUser {
        username: None,
        password: Some(hashed_password),
        salt: None,
        email: None,
        bio: None,
        is_private: None,
        display_name: None,
    };
    match update_user_by_id(&mut conn, user.id, &update_user).await {
        Ok(_) => {}
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete the email request
    match delete_request(&mut conn, user.id).await {
        Ok(_) => {}
        Err(e) => return e.into_response(),
    };

    init_session(req.session_mut(), user.id, &user.username);

    return Response::empty().into_response();
}
