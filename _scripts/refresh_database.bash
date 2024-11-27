#!/bin/bash
source .env
POSTGRES_dbname=postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@db:5432/$POSTGRES_DB

pg_dump --format=p --data-only --no-owner --dbname=$POSTGRES_dbname > database/backups/data.sql
diesel database reset
diesel migration run
psql $POSTGRES_dbname -f database/backups/data.sql