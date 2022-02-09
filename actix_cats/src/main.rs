use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct Cat {
    name: String,
    color: String,
    counter: Mutex<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Cat {
        name: String::from("Benny"),
        color: String::from("Brown"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_cats)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/cats")]
async fn get_cats(data: web::Data<Cat>) -> String {
    let name = &data.name;
    let color = &data.color;
    let mut counter = data.counter.lock().unwrap(); // get counter's mutex guard
    *counter += 1;

    format!(
        "Meow {}! Your fur is {}! Request no: {}!!",
        name, color, counter
    )
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
