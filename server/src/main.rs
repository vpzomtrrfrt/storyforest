#![feature(plugin)]
#![plugin(rocket_codegen)]

use serde_derive::Serialize;
#[macro_use]
extern crate diesel_derives;

#[derive(Queryable, Serialize)]
struct Tree {
    id: i32,
    name: String
}

#[get("/trees/<id>")]
fn trees_get(id: i32, state: rocket::State<ServerState>) -> rocket_contrib::Json<Tree> {

}

struct ServerState {
    conn: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager>
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
