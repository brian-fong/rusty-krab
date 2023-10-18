use actix_web::HttpResponse;
use uuid::Uuid;


pub async fn check_health() -> HttpResponse {
    let request_id = Uuid::new_v4();
    tracing::info!("Request #{} received", request_id);
    HttpResponse::Ok().finish()
}
