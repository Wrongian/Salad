use crate::{
    db::start_connection,
    funcs,
    routes::auth::{register, RegisterParams},
};
use diesel::PgConnection;
use tide::http::{Body, Method, Request, Response, Url};
use validator::Validate;

// Do not run this in production

// to run this with println! outputs use "rust test -- --nocapture"

// test whether can connect to the db
#[async_std::test]
async fn db_connection_test() -> tide::Result<()> {
    let conn: PgConnection = start_connection().await;
    Ok(())
}

// test whether can register
/* 
#[async_std::test]
async fn register_test() -> tide::Result<()> {
    let conn: PgConnection = start_connection().await;
    let url_string = "http://".to_string() + &funcs::get_url() + "/login";
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
    // returns ok if case is handled
    let res = register(req.into()).await;
    assert!(res.is_ok());
    Ok(())
}
*/