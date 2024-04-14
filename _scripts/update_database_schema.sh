# This script will create a new schema file based on the current database state
# This probably isn't needed except in strange situations like recovery or rebuild
diesel print-schema > database/src/schema.rs