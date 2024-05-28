use crate::{db::start_connection, lib, routes::auth::{register, RegisterParams}};
use diesel::PgConnection;
use tide::http::{Url, Method, Body, Request, Response};
use validator::Validate;

// Do not run this in production

#[async_std::test]
async fn db_connection_test() -> tide::Result<()> {
    let conn : PgConnection = start_connection().await;
    Ok(())
}

#[async_std::test]
async fn register_test() -> tide::Result<()> {
    let conn : PgConnection = start_connection().await;
    let mut app = tide::new();
    let url_string = "http://".to_string() + &lib::get_url();
    let url = Url::parse(&url_string)?;
    let mut req = Request::new(Method::Post, url);
    let register_params = RegisterParams {
        email: "bruh@gmail.com".to_string(),
        username: "lowtiergod".to_string(),
        password: "barnacles".to_string(),
    };
    // validate that it is correct
    register_params.validate()?;
    let body = Body::from_json(&register_params)?;
    req.set_body(body);
    let mut res: Response = app.respond(req).await?;
    println!("{}",res.body_string().await?);
    Ok(())
}

