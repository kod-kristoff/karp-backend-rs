#!/usr/bin/env bash
set -x
set -eo pipefail

# if ! [ -x "$(command -v mysql)" ]; then
#   echo >&2 "Error: mysql is not installed."
# exit 1
# fi
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.5.10 sqlx-cli --no-default-features --features mysql"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'mariadb'
DB_USER=${MARIADB_USER:=mariadb}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${MARIADB_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'karp'
DB_NAME="${MARIADB_DB:=karp}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${MARIADB_PORT:=23306}"

if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
      --detach \
      -e MYSQL_RANDOM_ROOT_PASSWORD=1 \
      -e MARIADB_USER=${DB_USER} \
      -e MARIADB_PASSWORD=${DB_PASSWORD} \
      -e MARIADB_DATABASE=${DB_NAME} \
      -p "${DB_PORT}":3306 \
      --name mariadb_karp_rs \
      mariadb:10.7 # -N 1000 # ^ Increased maximum number of connections for testing purposes
fi

function is_mysql_alive() {
  docker exec -it mariadb_karp_rs \
    mysqladmin ping \
      --user="${DB_USER}" \
      --password="${DB_PASSWORD}" \
      --host=${CONTAINER_DB_HOST} \
      --port=${DB_PORT} \
    > /dev/null
  returned_value=$?
  echo "${returned_value}"
}

until [ "$(is_mysql_alive)" -eq 0 ]
do
  sleep 2
  echo "Waiting for MySQL to be ready..."
done

# export MYSQL_PWD="${DB_PASSWORD}"
# until mysql -h "localhost" -P${DB_PORT} -u"${DB_USER}" -p"${DB_PASSWORD}" mariadb -c '\q'; do
#     >&2 echo "Mariadb is still unavailable - sleeping"
#     sleep 1
# done

>&2 echo "Mariadb is up and running on port ${DB_PORT}"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@127.0.0.1:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Mariadb has been migrated, ready to go!"
