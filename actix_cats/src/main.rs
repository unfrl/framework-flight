#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .service(get_cat)
    })
    .bind(&bind)?
    .run()
    .await
}

#[get("/cats/{cat_id}")]
async fn get_cat(pool: web::Data<DbPool>, cat_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let cat_id = cat_id.into_inner();

    // use web::block to offload blocking b/c diesel doesn't support tokio
    let cat = web::block(move || {
        let conn = pool.get()?;
        actions::get_cat_by_id(cat_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(cat) = cat {
        Ok(HttpResponse::Ok().json(cat))
    } else {
        let res = HttpResponse::NotFound().body(format!("No cat found with id {}", cat_id));
        Ok(res)
    }
}
