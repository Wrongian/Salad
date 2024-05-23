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
use crate::db::{start_connection, user::get_user_id_from_name, user::get_password_salt_from_id};
use crate::db::user::create;
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

pub async fn login(mut req: Request<()>) -> tide::Result {
    let LoginParams {username, password} = req.body_json().await?;
    // probably need to validate this at some point
    let uid: i32 = req.session().get("user_id").expect("user_id not found in session");

    // start a connection with the database
    let mut conn = start_connection().await;

    // get the password hash from db
    let (password_hash, salt) = get_password_salt_from_id(&mut conn, uid).await;

    let salt_string = SaltString::encode_b64(salt.as_bytes())?;
    // verify password
    if verify_password(&password, &salt_string, &password_hash) {
        // password correct

        // login the user
        let user_id = get_user_id_from_name(&mut conn, &username).await;
        
        let session = req.session_mut();

        // insert user_id into the session
        session.insert("user_id", user_id)?;


        return Ok(format!("result: {}\n err: {}", true, "Successfully logged in").into());
    }
    else {

        // build response
        let res_body = StandardBody {
            result : false,
            err : "Incorrect password".to_string(),
        };
        let mut response = Response::builder(400)
            .body(tide::Body::from_form(&res_body)?)
            .build();

        return Ok(response)
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

    let user = create(&mut conn, &new_user).await;
    let user_id = get_user_id_from_name(&mut conn, &username).await;
    
    // log the user in
    let session = req.session_mut();

    // insert user_id into the session
    session.insert("user_id", user_id)?;

    Ok(format!("result: {}\n err: {}", true, "Successfully Registered").into())
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
            if hash.to_string() == *hash_string {
                return true;
            }
            return false;
        }
        Err(e) => {
            return false;
        }
    }
    false
}