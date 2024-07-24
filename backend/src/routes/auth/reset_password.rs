use crate::connectors::db::connection::DBConnection;
use crate::connectors::db::reset::{
    create_request, get_request_by_id, replace_request, request_exists,
};
use crate::connectors::db::user::get_user_by_id;
use crate::connectors::db::user::update_user_by_id;
use crate::connectors::smtp::smtp_service::SMTPService;
use crate::helpers::auth::get_session_user_id;
use crate::helpers::random::make_random_string;
use crate::models::reset::{InsertRequest, UpdateRequest};
use crate::models::users::UpdateUser;
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

// less important less cost
// consts
const COST: u32 = 8;
const HASH_LEN: usize = 32;
const RESET_PASSWORD_SUBJECT: Lazy<String> = Lazy::new(|| "Saladify Password Reset".to_string());
const RESET_PASSWORD_BODY: Lazy<String> = Lazy::new(|| "Your Verification Code is:".to_string());

// structs
#[derive(Deserialize, Validate, Debug)]
struct PasswordCodeParams {
    code: String,
}

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
}

// the get route for reset-password
// user requests to send the password then an email is sent to their email
// a request obj is created in the database which stores information related to this password reset request
// if the request object already exists then replace it
pub async fn get_email(req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get state
    let state = req.state();

    // get connection pool
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // make random code
    let code = make_random_string(HASH_LEN);
    // hash random code
    let hashed_code = match hash(&code, COST) {
        Ok(new_code) => new_code,
        Err(e) => return Error::HashError(e).into_response(),
    };

    // get the users email
    let user_email = match get_user_by_id(&mut conn, user_id).await {
        Ok(u) => u,
        Err(e) => return Error::DieselError(e).into_response(),
    }
    .email;

    // get smtp service
    let email_service: Box<&(dyn SMTPService + Sync)> = Box::new(&state.email_service);
    //  send email
    // in the future could build html using handlebar or something
    match (*email_service).send_email(
        user_email,
        (*RESET_PASSWORD_SUBJECT).to_owned(),
        (*RESET_PASSWORD_BODY).to_owned() + &code,
    ) {
        Ok(_) => {}
        Err(e) => return e.into_response(),
    };

    // check if reset request already exists
    let already_exists: bool = match request_exists(&mut conn, user_id).await {
        Ok(is_true) => is_true,
        Err(e) => return e.into_response(),
    };

    if already_exists {
        // replace
        let new_request = UpdateRequest {
            user_id: None,
            code: hashed_code,
        };
        match replace_request(&mut conn, user_id, new_request).await {
            Ok(_) => {}
            Err(e) => return e.into_response(),
        };
    } else {
        // make new one
        let new_request = InsertRequest {
            user_id: user_id,
            code: hashed_code,
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
    // get user id from session
    // if not then user is not logged in
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get payload
    let code_params: PasswordCodeParams = match req.body_json().await {
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

    // get the code
    let code = match get_request_by_id(&mut conn, user_id).await {
        Ok(request) => request.code,
        Err(e) => return e.into_response(),
    };

    match verify(code_params.code, &code) {
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
    // get user id from session
    // if not then user is not logged in
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

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

    // get the code
    let code = match get_request_by_id(&mut conn, user_id).await {
        Ok(request) => request.code,
        Err(e) => return e.into_response(),
    };

    // if code  doesnt match
    if code != reset_params.code {
        return Error::WrongPasswordResetCodeError().into_response();
    }

    // update user
    let update_user = UpdateUser {
        username: None,
        password: Some(reset_params.password),
        salt: None,
        email: None,
        bio: None,
        is_private: None,
        display_name: None,
    };
    match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(_) => {}
        Err(e) => return Error::DieselError(e).into_response(),
    };

    return Response::empty().into_response();
}
