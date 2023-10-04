#!/usr/bin/env bash
# set -x
set -eo pipefail

# Postgres credentials
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}

# Launch docker container with postgres image
if [[ -z "${SKIP_DOCKER}" ]]
then
  echo "Launching Docker container..."
  container_id=$(docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}:5432" \
    -d postgres \
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
  -h "localhost" \
  -U ${DB_USER} \
  -p ${DB_PORT} \
  -d ${DB_NAME} \
  -c "\q" \
  2>/dev/null; do
    >&2 echo "Postgres is loading..."
    sleep 1
done

>&2 echo "Postgres is up and running on port: ${DB_PORT}"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

# Create postgres database
# sqlx database create

# Run migrations
sqlx migrate run

# # Stop and remove all containers
# docker ps -aq | xargs docker stop | xargs docker rm 2>/dev/null