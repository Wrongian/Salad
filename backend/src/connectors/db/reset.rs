use crate::models::reset::{GetRequest, InsertRequest};
use crate::types::error::Error;
use crate::types::error::Error::DieselError;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

/*
which queries should be inside?
It should be able to accomodate
1. verify that a request exists
2. retrieve a request
3. check to make sure a request is not expired
4. delete a request once its done
5. create a new request
6. update a request with a new one
*/

/*
Links should have
create
update
delete
 */
pub async fn create_request(
    conn: &mut PgConnection,
    request: InsertRequest,
) -> Result<GetRequest, Error> {
    use crate::schema::reset_password_request;
    match diesel::insert_into(reset_password_request::table)
        .values(request)
        .returning(GetRequest::as_returning())
        .get_result(conn)
    {
        Ok(req) => return Ok(req),
        Err(e) => return Err(DieselError(e)),
    }
}

pub async fn get_request_by_id(conn: &mut PgConnection, uid: i32) -> Result<GetRequest, Error> {
    use crate::schema::reset_password_request::dsl::*;
    // only one request per user
    match reset_password_request
        .filter(user_id.eq(uid))
        .select(GetRequest::as_select())
        .first::<GetRequest>(conn)
    {
        Ok(req) => return Ok(req),
        Err(e) => return Err(DieselError(e)),
    }
}

pub async fn request_exists(conn: &mut PgConnection, uid: i32) -> Result<bool, Error> {
    use crate::schema::reset_password_request::dsl::*;
    match reset_password_request
        .filter(user_id.eq(uid))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
    {
        Ok(count) => return Ok(count),
        Err(e) => return Err(DieselError(e)),
    }
}

pub async fn replace_request(
    conn: &mut PgConnection,
    uid: i32,
    new_request: InsertRequest,
) -> Result<(), Error> {
    /*
    use crate::schema::reset_password_request::dsl::*;
    match diesel::update(reset_password_request.filter(user_id.eq(uid)))
        .set(new_request)
        .returning(id)
        .get_result::<i32>(conn)
    {
        Ok(_) => return Ok(()),
        Err(e) => return Err(DieselError(e)),
    }
    */
    match delete_request(conn, uid).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    let new_request = InsertRequest {
        code: new_request.code,
        user_id: uid,
        created_at: new_request.created_at,
    };

    match create_request(conn, new_request).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    };
    Ok(())
}

pub async fn delete_request(conn: &mut PgConnection, uid: i32) -> Result<(), Error> {
    use crate::schema::reset_password_request::dsl::*;
    match diesel::delete(reset_password_request.filter(user_id.eq(uid)))
        .returning(id)
        .get_result::<i32>(conn)
    {
        Ok(_) => return Ok(()),
        Err(e) => return Err(DieselError(e)),
    }
}
