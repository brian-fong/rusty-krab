use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::debug!(
        "Request #{}: new subscription: {} ({})...",
        request_id, form.name, form.email
    );

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
    .await
    {
        Ok(_) => {
            log::debug!(
                "Request #{}: subscription created successfully",
                request_id
            );
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!(
                "Request #{}: failed to execute query: {:#?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
