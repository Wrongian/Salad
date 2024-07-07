use std::sync::Arc;

use crate::{
    connectors::db::{
        connection::DBConnection,
        user::{check_user_exists, create, get_user_id_from_name},
    },
    models::users::InsertUser,
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};
use bcrypt::hash;
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tide::Request;
use validator::{Validate, ValidationError};

use super::init_session;

// password cost
const COST: u32 = 10;

// regex for password
const PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| { Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$") }.unwrap());

fn validate_password(value: &str) -> Result<(), ValidationError> {
    if (&*PASSWORD_REGEX).is_match(value).unwrap() {
        return Ok(());
    }
    Err(ValidationError::new("Invalid Password"))
}

#[derive(Debug, Deserialize, Validate, Serialize)]
// register parameters for register route
pub struct RegisterParams {
    #[validate(email(message = "Email is incorrect"))]
    pub email: String,
    #[validate(length(max = 30, message = "Username must be between 5 to 30 characters"))]
    pub username: String,
    #[validate(
        length(
            min = 8,
            max = 50,
            message = "Password must be between 8 to 50 characters"
        ),
        custom(
            function = "validate_password",
            message = "Password must have at least one letter and one number"
        )
    )]
    pub password: String,
}

// handles the requests coming into the register route and gives back an appropriate response
pub async fn register(mut req: Request<Arc<TideState>>) -> tide::Result {
    // process the register request body
    let register_params: RegisterParams;
    match req.body_json().await {
        Ok(params) => {
            register_params = params;
        }
        Err(e) => {
            log::error!("Error converting request body to json: {}", e);
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response();
        }
    }

    // borrow fields
    let email = &register_params.email;
    let username = &register_params.username;
    let password = &register_params.password;

    match register_params.validate() {
        Ok(_) => (),
        Err(e) => return Error::ValidationError(e).into_response(),
    }
    // hash salt
    let password_res = hash(password, COST);
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

    // create new user instance
    let new_user = InsertUser {
        username: username.clone(),
        password: hashed_password,
        email: email.clone(),
        is_private: false,
        bio: None,
        salt: "".to_string(),
        display_name: username.clone(),
    };

    // start a connection with the database
    let state: Arc<TideState> = req.state().clone();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // check if the user already exists
    if check_user_exists(&mut conn, &username, &email).await {
        return Error::DuplicateDBError("Username or Email".to_string()).into_response();
    }

    // create user
    let user = create(&mut conn, &new_user).await;
    let user_id = get_user_id_from_name(&mut conn, &username).await;

    // log the user in
    // insert user_id, username into the session
    init_session(req.session_mut(), user_id, &username);

    // all done
    return Response::empty().into_response();
}
