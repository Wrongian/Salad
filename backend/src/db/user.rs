struct User {
    username: String,
    // hashed password
    password: String,  
    email: String,
    bio: String,
    is_private: bool
}

async fn create(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO users (username, password, email, bio, is_private) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.email)
        .bind(&user.bio)
        .bind(&user.is_private)
        .execute(pool)
        .await?;
    Ok(());
}