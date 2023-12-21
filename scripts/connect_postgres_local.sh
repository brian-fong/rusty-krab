#!/bin/bash
set -eo pipefail

# Postgres credentials
HOST="localhost"
DB_USER=postgres
DB_PASSWORD=password
DB_NAME=newsletter
DB_PORT=5432

# Connect to postgres database
export PGPASSWORD=${DB_PASSWORD}
psql \
  -h ${HOST} \
  -U ${DB_USER} \
  -p ${DB_PORT} \
  -d ${DB_NAME} \
