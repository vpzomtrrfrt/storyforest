use crate::{Error, ServerState};

use bcrypt;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct UserPostQuery {
    pub username: String,
    pub password: String
}

#[post("/users", data = "<query>")]
fn users_post(
    query: rocket_contrib::Json<UserPostQuery>,
    state: rocket::State<ServerState>
) -> Result<rocket_contrib::Json<i32>, Error> {
    let rocket_contrib::Json(UserPostQuery { username, password }) = query;
    let passhash = bcrypt::hash(&password, 10)?;

    let conn = state.conn.get()?;
    let res = {
        use crate::schema::account::dsl;
        diesel::insert_into(dsl::account)
            .values((
                    dsl::name.eq(username),
                    dsl::passhash.eq(passhash)
            ))
            .returning(dsl::id)
            .get_result(&conn)
    }?;
    Ok(rocket_contrib::Json(res))
}

#[post("/logins", data = "<query>")]
fn logins_post(
    query: rocket_contrib::Json<UserPostQuery>,
    state: rocket::State<ServerState>,
    mut cookies: rocket::http::Cookies
) -> Result<rocket_contrib::Json<i32>, Error> {
    let rocket_contrib::Json(UserPostQuery { username, password }) = query;

    #[derive(Queryable)]
    struct Res {
        id: i32,
        passhash: Option<String>,
    }
    let res: Res = {
        let conn = state.conn.get()?;
        use crate::schema::account::dsl;
        dsl::account
            .filter(dsl::name.eq(username))
            .select((dsl::id, dsl::passhash))
            .first(&conn)
    }?;

    let correct = match res.passhash {
        Some(hash) => bcrypt::verify(&password, &hash)?,
        None => false
    };
    if !correct {
        return Err(Error::Forbidden("Incorrect password.".to_owned()));
    }

    cookies.add_private(rocket::http::Cookie::new("user_id", res.id.to_string()));

    Ok(rocket_contrib::Json(res.id))
}
