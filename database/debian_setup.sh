#!/usr/bin/env bash
set -e

db_pass=${DB_PASS:?"Error: DB_PASS must be provided."}
db_name=${DB_NAME:-'dev'}
db_user=${DB_USER:-'user'}
allowed_cidr=${DB_ALLOWED_CIDR:-'10.0.0.0/8'}

apt update && apt upgrade -y
apt install -y postgresql

su - postgres -c "psql -c \"CREATE USER $db_user WITH PASSWORD '$db_pass';\""
su - postgres -c "psql -c \"CREATE DATABASE $db_name;\""
su - postgres -c "psql -c \"GRANT ALL PRIVILEGES ON DATABASE $db_name TO $db_user;\""
# TODO: test the following lines
su - postgres -c "psql -d $db_name -c \"GRANT USAGE, CREATE ON SCHEMA public TO $db_user;\""
su - postgres -c "psql -d $db_name -c \"GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO $db_user;\""
su - postgres -c "psql -d $db_name -c \"GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO $db_user;\""
su - postgres -c "psql -d $db_name -c \"ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO $db_user;\""
su - postgres -c "psql -d $db_name -c \"ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON SEQUENCES TO $db_user;\""

PG_VERSION=$(psql --version | grep -oE '[0-9]+' | head -1)
PG_CONF_DIR="/etc/postgresql/$PG_VERSION/main"

su - postgres -c "echo \"listen_addresses = '0.0.0.0'\" >> $PG_CONF_DIR/postgresql.conf"
su - postgres -c "echo \"host $db_name $db_user $allowed_cidr scram-sha-256\" >> $PG_CONF_DIR/pg_hba.conf"

systemctl restart postgresql

echo "Created a database with the following configurations:"
echo "  DB Name: $db_name"
echo "  DB User: $db_user"
echo "  DB Allowed subnet in CIDR: $allowed_cidr"

