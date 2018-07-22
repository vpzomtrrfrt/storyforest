#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate quick_error;

use serde_derive::Serialize;
use diesel::deserialize::Queryable;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};

mod schema;

quick_error! {
    #[derive(Debug)]
    enum Error {
        PoolError(e: diesel::r2d2::PoolError) { from() }
        DBError(e: diesel::result::Error) { from() }
        NotFound(e: String)
        Internal(e: String)
    }
}

impl<'r> rocket::response::Responder<'r> for Error {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        match self {
            Error::NotFound(e) => {
                rocket::Response::build()
                    .status(rocket::http::Status::NotFound)
                    .header(rocket::http::ContentType::Plain)
                    .sized_body(std::io::Cursor::new(e))
                    .ok()
            },
            _ => {
                Err(rocket::http::Status::InternalServerError)
            }
        }
    }
}

#[derive(Queryable, Serialize)]
struct RootNode {
    id: i32,
    text: String
}

#[derive(Queryable, Serialize)]
struct Tree {
    id: i32,
    name: String
}

#[derive(Queryable, Serialize)]
struct TreeDetail {
    id: i32,
    name: String,
    roots: Vec<RootNode>
}

#[get("/trees/<id>")]
fn trees_get(id: i32, state: rocket::State<ServerState>) -> Result<rocket_contrib::Json<TreeDetail>, Error> {
    let conn = state.conn.get()?;
    let res1 = {
        use self::schema::tree::dsl;
        dsl::tree
        .filter(dsl::id.eq(id))
        .load::<Tree>(&conn)
    }?;
    let res2 = {
        use self::schema::node::dsl;
        dsl::node
            .filter(dsl::tree.eq(id))
            .filter(dsl::parent.is_null())
            .select((dsl::id, dsl::text))
            .load::<RootNode>(&conn)
    }?;
    if res1.len() < 1 {
        Err(Error::NotFound("No such tree".to_owned()))
    }
    else if res1.len() > 1 {
        Err(Error::Internal("More than one tree returned for single ID".to_owned()))
    }
    else {
        let tree = res1.into_iter().next().unwrap();
        let tree = TreeDetail {
            id: tree.id,
            name: tree.name,
            roots: res2
        };
        Ok(rocket_contrib::Json(tree))
    }
}

struct ServerState {
    conn: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>
}

fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Missing DATABASE_URL");

    rocket::ignite()
        .manage(ServerState {
            conn: diesel::r2d2::Pool::builder()
                .build(diesel::r2d2::ConnectionManager::new(database_url))
                .expect("Failed to construct connection pool")
        })
        .mount("/", routes![trees_get])
        .launch();
}
