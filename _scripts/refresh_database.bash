#!/bin/bash
source .env
POSTGRES_dbname=postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@db:5432/$POSTGRES_DB

pg_dump --format=p --data-only --no-owner --dbname=$POSTGRES_dbname > database/backups/data.sql
diesel database reset
diesel migration run
psql $POSTGRES_dbname -f database/backups/data.sql
# pg_restore --dbname=$POSTGRES_dbname -c --format=p --schema-only database/backups/data.sql

# psql $POSTGRES_dbname -f database/backups/backup_1_data.sql