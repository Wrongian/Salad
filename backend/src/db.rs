pub mod user;
use dotenv::dotenv;
use sqlx::Postgres;
use std::env;
use sqlx::Pool;


pub async fn start_connection() -> tide::Result<Pool<Postgres>> {
    dotenv().expect("No .env file found");
    
    // setup database
    let db_host = env::var("POSTGRES_HOST").expect("");
    let db_port = env::var("POSTGRES_PORT").expect("");
    let db_user = env::var("POSTGRES_USER").expect("");
    let db_password = env::var("POSTGRES_PASSWORD").expect("");
    let db_name = env::var("POSTGRES_NAME").expect("");

    // database url
    let database_url = db_host.clone() + "://" + &db_user + ":" + &db_password + "@localhost:" + &db_port + "/" + &db_name;

    // create pool
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    // migrate
    // placeholder for now
    // sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
