use crate::db::start_connection;
use crate::db::user::{
    check_user_exists, check_username_present, create, get_password_salt_from_id,
    get_user_id_from_name,
};
use crate::models::users::User;
use bcrypt::verify;
use bcrypt::hash;
use std::borrow::Borrow;
use tide::prelude::*;
use tide::Request;
use tide::Response;
use validator::Validate;

// password cost
const COST : u32 = 10;

#[derive(Debug, Deserialize, Validate, Serialize)]
// register parameters for register route
pub struct RegisterParams {
    #[validate(email(message = "Email is incorrect"))]
    pub email: String,
    #[validate(length(max = 30, message = "Username must be between 5 to 30 characters"))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 to 50 characters"
    ))]
    pub password: String,
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

// standard response body without error
#[derive(Debug, Serialize)]
pub struct StandardBody {
    pub result: bool,
    pub err: String,
}

// build a tide result with standard response body
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

// handles the requests coming into the login route and gives back an appropriate response
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

    // get fields
    let username = &login_params.username;
    let password = &login_params.password;

    // validate login parameters
    match login_params.validate() {
        Err(e) => {
            // returns the validation errors
            let mut error_string: String = "".to_string();
            let validations = e.field_errors();
            let values = validations.values();
            for validation_errors in values {
                for validation_error in validation_errors.iter() {
                    let error_message = validation_error.message.borrow();
                    match error_message {
                        Some(message) => {
                            error_string += message.borrow();
                            error_string += ".";
                        }
                        None => {}
                    }
                }
            }

            return build_response(false, error_string, 400);
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

    // verify the password is correct
    // bcrypt alr auto includes the hash into the password
    let verify_password_res = verify(&password, &password_hash);
    match verify_password_res {
        Ok(is_correct) => {
            if is_correct {
                // password correct

                // login the user
                let user_id = get_user_id_from_name(&mut conn, &username).await;

                let session = req.session_mut();

                // insert user_id into the session
                session.insert("user_id", user_id)?;

                return build_response(true, "".to_string(), 200);
            }
            else {
                // password is incorrect
                return build_response(false, "Incorrect Password".to_string(), 400);
            }
   
        }
        Err(e) => {
            // log the error
            println!("Error has occurred: {}", e.to_string());
            // Returns a response that does not expose internal implementation
            return build_response(false, "".to_string(), 500);
        }
    }
}

// handles the requests coming into the register route and gives back an appropriate response
pub async fn register(mut req: Request<()>) -> tide::Result {
    // process the register request body
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

    // borrow fields
    let email = &register_params.email;
    let username = &register_params.username;
    let password = &register_params.password;

    // validate register
    match register_params.validate() {
        Err(e) => {
            // returns the validation error
            let mut error_string: String = "".to_string();
            let validations = e.field_errors();
            let values = validations.values();
            for validation_errors in values {
                for validation_error in validation_errors.iter() {
                    let error_message = validation_error.message.borrow();
                    match error_message {
                        Some(message) => {
                            error_string += message.borrow();
                            error_string += ".";
                        }
                        None => {}
                    }
                }
            }
            return build_response(false, error_string, 400);
        }

        _ => (),
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
            return build_response(false, "".to_string(), 500);
        }
    } 

    // create new user instance
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

    // all done
    return build_response(true, "".to_string(), 200);
}



// legacy code for old password hashing
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
