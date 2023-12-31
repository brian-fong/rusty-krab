# Commit Messages
🔁 refactor: code cleanup
- startup.rs
- tests/subscriptions.rs

# Deploying to Digital Ocean
- Create project: `doctl apps create --spec app-spec.yaml`
- Migrate database: `DATABASE_URL=<digital_ocean_db_connection_string> sqlx migrate run`
