#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate quick_error;
extern crate bcrypt;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

mod schema;
mod paths;

quick_error! {
    #[derive(Debug)]
    enum Error {
        PoolError(e: diesel::r2d2::PoolError) { from() }
        DBError(e: diesel::result::Error) { from() }
        BcryptError(e: bcrypt::BcryptError) { from() }
        NotFound(e: String)
        Internal(e: String)
    }
}

impl<'r> rocket::response::Responder<'r> for Error {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        match self {
            Error::NotFound(e) => rocket::Response::build()
                .status(rocket::http::Status::NotFound)
                .header(rocket::http::ContentType::Plain)
                .sized_body(std::io::Cursor::new(e))
                .ok(),
            _ => Err(rocket::http::Status::InternalServerError),
        }
    }
}

#[derive(Queryable, Serialize)]
struct TreeNodeQuery {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize)]
struct TreeNode {
    id: i32,
    text: String,
    children: Option<Vec<TreeNode>>,
}

#[derive(Queryable, Serialize)]
struct Tree {
    id: i32,
    name: String,
}

#[derive(Queryable, Serialize)]
struct TreeDetail {
    id: i32,
    name: String,
    roots: Vec<TreeNode>,
}

#[derive(Queryable)]
struct NodeChild {
    id: i32,
    text: String,
    parent: Option<i32>,
}

#[derive(Serialize)]
struct NodeResult {
    id: i32,
    text: String,
    children: Vec<TreeNode>,
    parent: Option<TreeNodeQuery>
}

fn get_children(parents: &[i32], conn: &diesel::pg::PgConnection) -> Result<Vec<NodeChild>, Error> {
    use self::schema::node::dsl;
    dsl::node
        .filter(dsl::parent.eq(diesel::dsl::any(parents)))
        .select((dsl::id, dsl::text, dsl::parent))
        .load(conn)
        .map_err(|e| e.into())
}

#[get("/trees/<id>")]
fn trees_get(
    id: i32,
    state: rocket::State<ServerState>,
) -> Result<rocket_contrib::Json<TreeDetail>, Error> {
    let conn = state.conn.get()?;
    let res1 = {
        use self::schema::tree::dsl;
        dsl::tree.filter(dsl::id.eq(id))
            .select((dsl::id, dsl::name))
            .load::<Tree>(&conn)
    }?;
    let res2 = {
        use self::schema::node::dsl;
        dsl::node
            .filter(dsl::tree.eq(id))
            .filter(dsl::parent.is_null())
            .select((dsl::id, dsl::text))
            .load::<TreeNodeQuery>(&conn)
    }?;
    let res3 = get_children(&res2.iter().map(|q| q.id).collect::<Vec<_>>(), &conn)?;
    let mut roots: Vec<_> = res2
        .into_iter()
        .map(|n| TreeNode {
            children: Some(Vec::new()),
            id: n.id,
            text: n.text,
        })
        .collect();
    for child in res3 {
        for root in roots.iter_mut() {
            if Some(root.id) == child.parent {
                if let Some(ref mut children) = root.children {
                    children.push(TreeNode {
                        id: child.id,
                        text: child.text,
                        children: None,
                    });
                    break;
                }
            }
        }
    }
    if res1.len() < 1 {
        Err(Error::NotFound("No such tree".to_owned()))
    } else if res1.len() > 1 {
        Err(Error::Internal(
            "More than one tree returned for single ID".to_owned(),
        ))
    } else {
        let tree = res1.into_iter().next().unwrap();
        let tree = TreeDetail {
            id: tree.id,
            name: tree.name,
            roots,
        };
        Ok(rocket_contrib::Json(tree))
    }
}

#[derive(Deserialize)]
struct NodePostQuery {
    pub parent: i32,
    pub text: String,
}

#[post("/nodes", data = "<query>")]
fn nodes_post(
    query: rocket_contrib::Json<NodePostQuery>,
    state: rocket::State<ServerState>,
) -> Result<rocket_contrib::Json<i32>, Error> {
    let conn = state.conn.get()?;
    let tree: i32 = {
        use self::schema::node::dsl;
        dsl::node.select(dsl::tree).first(&conn)
    }?;
    let res = {
        use self::schema::node::dsl;
        diesel::insert_into(dsl::node)
            .values((
                dsl::parent.eq(query.parent),
                dsl::tree.eq(tree),
                dsl::text.eq(query.text.to_owned()),
            ))
            .returning(dsl::id)
            .get_result(&conn)
    }?;
    Ok(rocket_contrib::Json(res))
}

#[get("/nodes/<id>")]
fn nodes_get(
    id: i32,
    state: rocket::State<ServerState>,
) -> Result<rocket_contrib::Json<NodeResult>, Error> {
    let conn = state.conn.get()?;
    let res1 = {
        use self::schema::node::dsl;
        dsl::node
            .filter(dsl::id.eq(id))
            .select((dsl::id, dsl::text, dsl::parent))
            .first::<NodeChild>(&conn)
    }?;
    let res2 = {
        use self::schema::node::dsl;
        dsl::node
            .filter(dsl::parent.eq(id))
            .select((dsl::id, dsl::text))
            .load::<TreeNodeQuery>(&conn)
    }?;
    let res3 = get_children(&res2.iter().map(|q| q.id).collect::<Vec<_>>(), &conn)?;
    let res4: Option<String> = match res1.parent {
        Some(parent) => {
            Some({
                use self::schema::node::dsl;
                dsl::node
                    .filter(dsl::id.eq(parent))
                    .select(dsl::text)
                    .first(&conn)
            }?)
        },
        None => None
    };

    let mut children = res2
        .into_iter()
        .map(|q| TreeNode {
            id: q.id,
            text: q.text,
            children: Some(Vec::new()),
        })
        .collect::<Vec<_>>();

    for child in res3 {
        for root in children.iter_mut() {
            if Some(root.id) == child.parent {
                if let Some(ref mut children) = root.children {
                    children.push(TreeNode {
                        id: child.id,
                        text: child.text,
                        children: None,
                    });
                    break;
                }
            }
        }
    }

    Ok(rocket_contrib::Json(NodeResult {
        parent: res4.map(|t| {
            TreeNodeQuery {
                id: res1.parent.unwrap(),
                text: t
            }
        }),
        id: res1.id,
        text: res1.text,
        children: children,
    }))
}

#[derive(Serialize)]
struct StoryNode {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize)]
struct Story {
    pub tree: i32,
    pub nodes: Vec<StoryNode>,
}

#[get("/nodes/<id>/story")]
fn nodes_story_get(
    id: i32,
    state: rocket::State<ServerState>,
) -> Result<rocket_contrib::Json<Story>, Error> {
    let conn = state.conn.get()?;
    let mut current_id = id;
    let mut nodes = Vec::new();
    #[derive(Queryable)]
    pub struct StoryNodeRes {
        text: String,
        parent: Option<i32>,
        tree: i32,
    }
    loop {
        let res: StoryNodeRes = {
            use self::schema::node::dsl;
            dsl::node
                .filter(dsl::id.eq(current_id))
                .select((dsl::text, dsl::parent, dsl::tree))
                .first(&conn)
        }?;
        nodes.push(StoryNode {
            id: current_id,
            text: res.text,
        });
        if let Some(parent) = res.parent {
            current_id = parent;
        } else {
            nodes.reverse();
            return Ok(rocket_contrib::Json(Story {
                tree: res.tree,
                nodes,
            }));
        }
    }
}

struct ServerState {
    conn: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>,
}

fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    rocket::ignite()
        .manage(ServerState {
            conn: diesel::r2d2::Pool::builder()
                .build(diesel::r2d2::ConnectionManager::new(database_url))
                .expect("Failed to construct connection pool"),
        })
        .mount(
            "/",
            routes![trees_get, nodes_post, nodes_story_get, nodes_get, paths::users::users_post],
        )
        .launch();
}
