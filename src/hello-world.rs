// hello-world example
use actix_web::{web, App, HttpServer, Responder};

async fn greet() -> impl Responder {
    format!("hello world!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
