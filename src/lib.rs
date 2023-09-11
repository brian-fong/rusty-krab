use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn check_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn start(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/checkhealth", web::get().to(check_health))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
