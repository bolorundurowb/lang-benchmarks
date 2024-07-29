use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 7654))?
    .run()
    .await;
}