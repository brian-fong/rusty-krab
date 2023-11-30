# rusty-krab

### ToDo
[ ] Run migrations as part of start-up process


### Deploying to Digital Ocean
- Create project: `doctl apps create --spec app-spec.yaml`
- Migrate database: `DATABASE_URL=<digital_ocean_db_connection_string> sqlx migrate run`
