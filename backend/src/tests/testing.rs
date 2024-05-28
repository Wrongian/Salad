use crate::db::start_connection;
use diesel::PgConnection;



#[async_std::test]
async fn db_connection_test() -> tide::Result<()> {
    let conn : PgConnection = start_connection().await;
    Ok(())
}
}

