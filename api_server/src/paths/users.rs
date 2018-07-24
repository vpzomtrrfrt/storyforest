use crate::{Error, ServerState};

use bcrypt;
use diesel::{ExpressionMethods, RunQueryDsl};
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
