pub mod login;
pub mod logout;
pub mod register;
pub mod reset_password;

// password cost
const PASSWORD_COST: u32 = 10;

// init session when logged in
pub fn init_session(session: &mut tide::sessions::Session, user_id: i32, username: &String) {
    session
        .insert("user_id", user_id)
        .expect("Error serializing user_id");
    session
        .insert("username", username)
        .expect("Error serializing username");
}
