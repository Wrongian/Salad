use core::panic;
use tide::Request;
use scrypt::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString
    },
    Scrypt
};
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct LoginParams {
    username : String,
    password : String,
}

pub async fn login(mut req: Request<()>) -> tide::Result {
    // probably will change later to form
    // todo
    let LoginParams {username, password} = req.body_json().await?;
    Ok(format!("Username: {}\n Password: {}", username, password).into())
} 

pub async fn register(mut req: Request<()>) -> tide::Result {
    // probably will change later to form
    // todo
    let LoginParams {username, password} = req.body_json().await?;
    Ok(format!("Username: {}\n Password: {}", username, password).into())
}

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