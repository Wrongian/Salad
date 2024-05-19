use std::error::Error;

pub struct User {
    pub username: String,
    // hashed password
    pub password: String,  
    pub email: String,
    pub bio: String,
    pub is_private: bool
}

pub async fn create(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO users (username, password, email, bio, is_private) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.email)
        .bind(&user.bio)
        .bind(&user.is_private)
        .execute(pool)
        .await?;
    Ok(())
}