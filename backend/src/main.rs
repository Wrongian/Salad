pub mod routes;
use routes::auth::login;
use routes::auth::register;

#[async_std::main]
async fn main() -> tide::Result<()>{
    let mut app = tide::new();
    app.at("/login").post(login);
    app.at("/register").post(register);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
