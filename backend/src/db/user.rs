use std::error::Error;

// should create a separate models folder
pub struct User {
    pub username: String,
    // hashed password
    pub password: String,  
    pub email: String,
    pub bio: Option<String>,
    pub is_private: bool,
    pub salt: String,
}

pub async fn create(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO users (username, password, email, bio, is_private, salt) VALUES ($1, $2, $3, $4, $5, $6)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.email)
        .bind(&user.bio)
        .bind(&user.is_private)
        .bind(&user.salt)
        .execute(pool)
        .await?;
    Ok(())
}