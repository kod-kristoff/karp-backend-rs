#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'mariadb'
DB_USER=${MARIADB_USER:=mariadb}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${MARIADB_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${MARIADB_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${MARIADB_PORT:=5432}"
# Launch mariadb using Docker
docker run \
    -e MARIADB_USER=${DB_USER} \
    -e MARIADB_PASSWORD=${DB_PASSWORD} \
    -e MARIADB_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d mariadb \
    mariadb -N 1000 # ^ Increased maximum number of connections for testing purposes
