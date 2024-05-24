use core::panic;
use tide::{log::start, Request};
use scrypt::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString
    },
    Scrypt
};
use tide::prelude::*;
use tide::Response;
use crate::db::user::{
    check_user_exists, create, get_password_salt_from_id, get_user_id_from_name
};
use crate::db::start_connection;
use crate::models::users::User;

#[derive(Debug, Deserialize)]
pub struct RegisterParams {
    pub email : String,
    pub username : String,
    pub password : String,
}

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub username : String,
    pub password : String,
}

#[derive(Debug, Serialize)]
pub struct StandardBody {
    pub result : bool,
    pub err : String,
}

fn build_response(result: bool, err: String, status: u16) -> tide::Result{
    // build response
    let res_body = StandardBody {
        result : result,
        err : err,
    };
    let response = Response::builder(status)
        .body(tide::Body::from_json(&res_body)?)
        .build();
    Ok(response)
}

pub async fn login(mut req: Request<()>) -> tide::Result {
    let LoginParams {username, password} = req.body_json().await?;

    // start a connection with the database
    let mut conn = start_connection().await;

    let uid = get_user_id_from_name(&mut conn, &username).await;

    // get the password hash from db
    let (password_hash, salt) = get_password_salt_from_id(&mut conn, uid).await;

    let salt_string = SaltString::from_b64(&salt)?;

    // check if already logged in 
    let is_logged_in: Option<i32> = req.session().get("user_id");
    if is_logged_in != None {
        return build_response(false, "Already Logged in".to_string(), 400);
    }

    // verify password
    if verify_password(&password, &salt_string, &password_hash) {
        // password correct

        // login the user
        let user_id = get_user_id_from_name(&mut conn, &username).await;
        
        let session = req.session_mut();

        // insert user_id into the session
        session.insert("user_id", user_id)?;

        return build_response(true, "".to_string(), 200);
    }
    else {

        return build_response(false, "Incorrect Password".to_string(), 400);
    }

} 

pub async fn register(mut req: Request<()>) -> tide::Result {
    let RegisterParams {email, username, password} = req.body_json().await?;
    // generate salt
    let salt: SaltString = generate_salt();
    // hash salt
    let hashed_password = hash_password(&password, &salt);
    // get SaltString as String
    let salt_str = salt.to_string();
    // get back the SaltString
    // let salt = SaltString::from_b64(salt_str);
    
    // create new user
    let new_user = User {
        username: username.clone(),
        password: hashed_password,
        email: email.clone(),
        is_private: false,
        bio: None,
        salt: salt_str,
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

//
fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}
fn hash_password(password: &String, salt: &SaltString) -> String {
    let pass_arr = password.as_bytes();
    let res = Scrypt.hash_password(pass_arr, salt);

    match res {
        Ok(hash) => {
            return hash.to_string();
        }
        Err(e) => {
            panic!("brick");
        }
    }
}

fn verify_password(to_check: &String, salt: &SaltString, hash_string: &String) -> bool {
    let pass_arr = to_check.as_bytes();
    let res = Scrypt.hash_password(pass_arr, salt);
    match res {
        Ok(hash) => {

            println!("PASSWORD:");
            println!("{}",hash.to_string());
            println!("{}",hash_string);
            if hash.to_string() == *hash_string {
                return true;
            }
            return false;
        }
        Err(e) => {
            println!("Error: {}",e.to_string());
            return false;
        }
    }
    false
}