#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hola, Platzi")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable not found");

    let conn = PgConnection::establish(&db_url).expect("Could not connect to DB");
    use self::schema::posts;
    use self::schema::posts::dsl::*;
    use self::models::{Post, NewPost};
    
    let _posts_result = posts.load::<Post>(&conn).expect("error al ejecutar la query");
    
    for post in _posts_result {
        println!("{:?}", post);
    }

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        App::new().service(hello_world).data(pool.clone())
    }).bind(("0.0.0.0", 9900)).unwrap().run().await
}

// fn main() {
//     // dotenv().ok();

//     // let db_url = env::var("DATABASE_URL").expect("db url variable not found");

//     // let conn = PgConnection::establish(&db_url).expect("Could not connect to DB");

//     // // let new_post = NewPost {
//     // //     title:"Mi tercer blogpost",
//     // //     body:"Este es el texto que contiene el 333 blog",
//     // //     slug:"tercer-post",
//     // // };

    // let _post: Post = diesel::insert_into(posts::table).values(&new_post).get_result(&conn).expect("Falló la insercion");

// }
