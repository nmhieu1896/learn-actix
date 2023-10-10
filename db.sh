#!/usr/bin/env bash

set -x
set -eo pipefail


DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=postgres}"
DB_PORT="${POSTGRES_PORT:=5433}"
DB_HOST="${POSTGRES_HOST:=localhost}"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!"