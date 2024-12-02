# This script will create a new schema file based on the current database state
# This probably isn't needed except in strange situations like recovery or rebuild
diesel print-schema --database-url sqlite://$DATABASE_URL > database/src/schema.rs
