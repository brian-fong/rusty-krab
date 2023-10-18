use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding new subscription",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    let _span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving subscription to database");

    // Insert new entry into database
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("Request #{}: subscription created successfully", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Request #{}: failed to execute query: {:#?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
