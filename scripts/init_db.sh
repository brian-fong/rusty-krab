#!/usr/bin/env bash
set -eo pipefail

# Postgres credentials
HOST=localhost
DB_USER=postgres
DB_PASSWORD=password
DB_NAME=newsletter
DB_PORT=5432

# Launch docker container with postgres image
if [[ -z "${SKIP_DOCKER}" ]]
then
  echo "Launching Docker container..."
  container_id=$(docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}:${DB_PORT}" \
    -d \
    postgres -N 1000
  )
  echo "Container ID: $container_id"
fi


# Check if postgresql (psql) is installed
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: postgresql is not installed"
  exit 1
fi

# Check if sqlx/sqlx-cli is installed
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed"
  exit 1
fi

# Keep pinging postgres until it's ready to accept commands
export PGPASSWORD=${DB_PASSWORD}
until psql \
  -h ${HOST} \
  -U ${DB_USER} \
  -p ${DB_PORT} \
  -d ${DB_NAME} \
  -c "\q" &>/dev/null; do
    >&2 echo "Postgres is loading..."
    sleep 1
done

echo "Postgres is up and running on port: ${DB_PORT}"

# Define postgres connection url
export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

# Create database (not necessary since docker run alredy created one)
echo "Creating database..."
sqlx database create

# Run migrations
echo "Running migrations..."
sqlx migrate run
