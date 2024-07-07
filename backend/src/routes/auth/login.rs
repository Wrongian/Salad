use std::sync::Arc;

use bcrypt::verify;
use serde::{Deserialize, Serialize};
use tide::Request;
use validator::Validate;

use crate::{
    connectors::db::{
        connection::DBConnection,
        user::{check_username_present, get_password_salt_from_id, get_user_id_from_name},
    },
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

use super::init_session;

#[derive(Serialize)]
pub struct ResultBody {
    result: bool,
}

// login parameters for the login route
#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(
        min = 5,
        max = 30,
        message = "Username must be between 5 to 30 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 to 50 characters"
    ))]
    pub password: String,
}

// handles the requests coming into the login route and gives back an appropriate response
pub async fn login(mut req: Request<Arc<TideState>>) -> tide::Result {
    // process the body
    let login_params: LoginParams;
    match req.body_json().await {
        Ok(params) => {
            login_params = params;
        }
        Err(e) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    }

    // get fields
    let username = &login_params.username;
    let password = &login_params.password;

    // validate login parameters
    match login_params.validate() {
        Ok(_) => (),
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    // start a connection with the database
    let state: Arc<TideState> = req.state().clone();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // check if the username is present in the database
    if !(check_username_present(&mut conn, &username).await) {
        return Error::NotFoundError("Username".to_string()).into_response();
    }

    // get the user ID
    let uid = get_user_id_from_name(&mut conn, &username).await;

    // get the password hash from db
    let (password_hash, _) = get_password_salt_from_id(&mut conn, uid).await;

    // verify the password is correct
    // bcrypt alr auto includes the hash into the password
    let verify_password_res = verify(&password, &password_hash);
    match verify_password_res {
        Ok(is_correct) => {
            if is_correct {
                // password correct

                // login the user
                let user_id = get_user_id_from_name(&mut conn, &username).await;

                // insert user_id into the session
                init_session(req.session_mut(), user_id, &username);

                return Response::empty().into_response();
            } else {
                // password is incorrect
                return Error::WrongPasswordError().into_response();
            }
        }
        Err(e) => {
            // log the error
            println!("Error has occurred: {}", e.to_string());
            // Returns a response that does not expose internal implementation
            return Error::HashError(e).into_response();
        }
    }
}

pub async fn is_logged_in(mut req: Request<Arc<TideState>>) -> tide::Result {
    let session = req.session_mut();
    // for now use username later used is_logged_in
    let res = session.get::<String>("username");
    if res.is_some() {
        return Response::new(ResultBody { result: true }).into_response();
    }
    // user not logged in
    return Response::new(ResultBody { result: false }).into_response();
}
