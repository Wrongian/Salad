use crate::db::start_connection;
use crate::db::user::{
    check_user_exists, check_username_present, create, get_password_salt_from_id,
    get_user_id_from_name,
};
use crate::models::users::User;
use core::panic;
use scrypt::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString,
    },
    Scrypt,
};
use sha256::{digest, try_digest};
use tide::prelude::*;
use tide::Response;
use tide::{log::start, Request};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct RegisterParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 30))]
    pub username: String,
    #[validate(length(min = 5, max = 50))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(max = 30))]
    pub username: String,
    #[validate(length(min = 5, max = 50))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct StandardBody {
    pub result: bool,
    pub err: String,
}

fn build_response(result: bool, err: String, status: u16) -> tide::Result {
    // build response
    let res_body = StandardBody {
        result: result,
        err: err,
    };
    let response = Response::builder(status)
        .body(tide::Body::from_json(&res_body)?)
        .build();
    Ok(response)
}

pub async fn login(mut req: Request<()>) -> tide::Result {
    // process the body
    let login_params: LoginParams;
    match req.body_json().await {
        Ok(params) => {
            login_params = params;
        }
        Err(e) => {
            return build_response(false, "Bad Request Body".to_string(), 400);
        }
    }
    let username = &login_params.username;
    let password = &login_params.password;

    // validate login
    match login_params.validate() {
        Err(e) => {
            // returns the validation error
            return build_response(false, e.to_string(), 400);
        }
        _ => (),
    }

    // start a connection with the database
    let mut conn = start_connection().await;

    // check if the username is present in the database
    if !(check_username_present(&mut conn, &username).await) {
        return build_response(false, "User does not exist".to_string(), 400);
    }

    // get the user ID
    let uid = get_user_id_from_name(&mut conn, &username).await;

    // get the password hash from db
    let (password_hash, salt) = get_password_salt_from_id(&mut conn, uid).await;

    // check if already logged in
    let is_logged_in: Option<i32> = req.session().get("user_id");
    if is_logged_in != None {
        return build_response(true, "".to_string(), 200);
    }

    // verify password
    if verify_password(&password, &password_hash) {
        // password correct

        // login the user
        let user_id = get_user_id_from_name(&mut conn, &username).await;

        let session = req.session_mut();

        // insert user_id into the session
        session.insert("user_id", user_id)?;

        return build_response(true, "".to_string(), 200);
    } else {
        return build_response(false, "Incorrect Password".to_string(), 400);
    }
}

pub async fn register(mut req: Request<()>) -> tide::Result {
    // process the body
    let register_params: RegisterParams;
    match req.body_json().await {
        Ok(params) => {
            register_params = params;
        }
        Err(e) => {
            log::error!("Error converting request body to json: {}", e);
            return build_response(false, "Bad Request Body".to_string(), 400);
        }
    }

    let email = &register_params.email;
    let username = &register_params.username;
    let password = &register_params.password;

    // validate register
    match register_params.validate() {
        Err(e) => {
            // returns the validation error
            return build_response(false, e.to_string(), 400);
        }
        _ => (),
    }
    // hash salt
    let hashed_password = hash_password(password);

    // create new user
    let new_user = User {
        username: username.clone(),
        password: hashed_password,
        email: email.clone(),
        is_private: false,
        bio: None,
        salt: "".to_string(),
        display_name: username.clone(),
    };

    // start a connection with the database
    let mut conn = start_connection().await;

    // check if the user already exists
    if check_user_exists(&mut conn, &username, &email).await {
        return build_response(true, "Username or Email already taken".to_string(), 400);
    }

    // create user
    let user = create(&mut conn, &new_user).await;
    let user_id = get_user_id_from_name(&mut conn, &username).await;

    // log the user in
    let session = req.session_mut();

    // insert user_id into the session
    session.insert("user_id", user_id)?;

    return build_response(true, "".to_string(), 200);
}

fn hash_password(password: &String) -> String {
    digest(password)
}

fn verify_password(password: &String, hash_string: &String) -> bool {
    let val = digest(password);
    if val == *hash_string {
        return true;
    }
    false
}
//
// fn generate_salt() -> SaltString {
//     SaltString::generate(&mut OsRng)
// }
// fn hash_password(password: &String, salt: &SaltString) -> String {
//     let pass_arr = password.as_bytes();
//     let res = Scrypt.hash_password(pass_arr, salt);

//     match res {
//         Ok(hash) => {
//             return hash.to_string();
//         }
//         Err(e) => {
//             panic!("brick");
//         }
//     }
// }

// fn verify_password(to_check: &String, salt: &SaltString, hash_string: &String) -> bool {
//     let pass_arr = to_check.as_bytes();
//     let res = Scrypt.hash_password(pass_arr, salt);
//     match res {
//         Ok(hash) => {
//             println!("PASSWORD:");
//             println!("{}", hash.to_string());
//             println!("{}", hash_string);
//             if hash.to_string() == *hash_string {
//                 return true;
//             }
//             return false;
//         }
//         Err(e) => {
//             println!("Error: {}", e.to_string());
//             return false;
//         }
//     }
//     false
// }
