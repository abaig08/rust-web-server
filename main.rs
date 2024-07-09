use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hi there!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
