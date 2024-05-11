pub mod routes;
use routes::auth::login;
use routes::auth::register;
use dotenv

#[async_std::main]
async fn main() -> tide::Result<()>{
    // create app
    let mut app = tide::new();

    // setup routes
    app.at("/login").post(login);
    app.at("/register").post(register);

    // attach to IP and port
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
