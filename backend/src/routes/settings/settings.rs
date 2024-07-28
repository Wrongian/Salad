use crate::connectors::db::user::{does_email_exist, does_username_exist};
use crate::helpers::state::get_connection;
use crate::routes::auth::PASSWORD_COST;
use crate::{
    connectors::db::user::update_user_by_id,
    helpers::auth::get_session_user_id,
    models::users::UpdateUser,
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};
use bcrypt::hash;
use serde::Deserialize;
use std::sync::Arc;
use tide::Request;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct UpdatePrivacyPayload {
    is_private: bool,
}

// login parameters for the login route
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordParams {
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 to 50 characters"
    ))]
    pub password: String,
}

// email params
#[derive(Debug, Deserialize, Validate)]
pub struct ChangeEmailParams {
    #[validate(email(message = "Email is incorrect"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangeUsernameParams {
    #[validate(length(max = 30, message = "Username must be between 5 to 30 characters"))]
    pub username: String,
}

// make profile private
pub async fn update_privacy(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get json body as UpdateProfilePayload
    let update_privacy: UpdatePrivacyPayload = match req.body_json().await {
        Ok(body) => body,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate
    match update_privacy.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }
    let update_user = UpdateUser {
        username: None,
        password: None,
        salt: None,
        email: None,
        is_private: Some(update_privacy.is_private),
        bio: None,
        display_name: None,
    };
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(_result) => Response::empty().into_response(),
        Err(err) => Error::DieselError(err).into_response(),
    };
}

// change password
pub async fn change_password(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get json body as UpdateProfilePayload
    let change_password: ChangePasswordParams = match req.body_json().await {
        Ok(body) => body,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate
    match change_password.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let password_res = hash(change_password.password, PASSWORD_COST);
    let hashed_password: String;
    match password_res {
        Ok(password_hash) => {
            hashed_password = password_hash;
        }
        Err(e) => {
            // log the error
            println!("Error has occurred: {}", e.to_string());
            // Returns a response that does not expose internal implementation
            return Error::HashError(e).into_response();
        }
    }

    let update_user = UpdateUser {
        username: None,
        password: Some(hashed_password),
        salt: None,
        email: None,
        is_private: None,
        bio: None,
        display_name: None,
    };
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(_result) => Response::empty().into_response(),
        Err(err) => Error::DieselError(err).into_response(),
    };
}

// change email
pub async fn change_email(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get json body as UpdateProfilePayload
    let change_email: ChangeEmailParams = match req.body_json().await {
        Ok(body) => body,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate
    match change_email.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let mut conn = get_connection(&mut req);

    match does_email_exist(&mut conn, change_email.email.clone()).await {
        Ok(exists) => {
            if exists {
                return Error::DuplicateEmailError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    let update_user = UpdateUser {
        username: None,
        password: None,
        salt: None,
        email: Some(change_email.email),
        is_private: None,
        bio: None,
        display_name: None,
    };
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(_result) => Response::empty().into_response(),
        Err(err) => Error::DieselError(err).into_response(),
    };
}
// change email
pub async fn change_username(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get json body as UpdateProfilePayload
    let change_username: ChangeUsernameParams = match req.body_json().await {
        Ok(body) => body,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate
    match change_username.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let mut conn = get_connection(&mut req);

    match does_username_exist(&mut conn, change_username.username.clone()).await {
        Ok(exists) => {
            if exists {
                return Error::DuplicateEmailError().into_response();
            }
        }
        Err(e) => return e.into_response(),
    }

    let update_user = UpdateUser {
        username: Some(change_username.username.clone()),
        password: None,
        salt: None,
        email: None,
        is_private: None,
        bio: None,
        display_name: None,
    };

    req.session_mut()
        .insert("username", change_username.username)
        .expect("Error serializing username");
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(_result) => Response::empty().into_response(),
        Err(err) => Error::DieselError(err).into_response(),
    };
}
