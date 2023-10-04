#!/bin/bash
# set -x
set -eo pipefail

# Postgres credentials
HOST="localhost"
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}

# Connect to postgres database
export PGPASSWORD=${DB_PASSWORD}
psql \
  -h ${HOST} \
  -U ${DB_USER} \
  -p ${DB_PORT} \
  -d ${DB_NAME} \
