use core::panic;
use tide::{log::start, Request};
use scrypt::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString
    },
    Scrypt
};
use tide::prelude::*;
use crate::db::{start_connection, user::get_user_id_from_name};
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

pub async fn login(mut req: Request<()>) -> tide::Result {
    // probably will change later to form
    // todo
    let LoginParams {username, password} = req.body_json().await?;
    Ok(format!("Username: {}\n Password: {}", username, password).into())
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

    Ok(format!("Username: {}\n Password: {}", username, password).into())
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

fn verify_password(to_check: &String, salt: &SaltString, hash_string: &String) -> bool{
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