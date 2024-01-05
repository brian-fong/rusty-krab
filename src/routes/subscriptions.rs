use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{Subscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct SubscriberForm {
    email: String,
    name: String,
}

impl TryFrom<SubscriberForm> for Subscriber {
    type Error = String;

    fn try_from(form: SubscriberForm) -> Result<Self, Self::Error> {
        let email = SubscriberEmail::parse(form.email)?;
        let name = SubscriberName::parse(form.name)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        email = %form.email,
        name = %form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<SubscriberForm>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                "Request #{}: failed to execute query",
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Inserting subscriber into database",
    skip(subscriber, pool)
)]
pub async fn insert_subscriber(
    subscriber: &Subscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
