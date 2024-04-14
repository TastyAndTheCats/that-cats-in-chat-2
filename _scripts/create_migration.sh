# Generates a new set of migrations by comparing the schema file to the database contents
#
# - Uses the first parameter as the migration name (e.g. `sh _scripts/create_migration.sh migration_name`)
#
# There are two ways to use this:
# 1) If you want to write the SQL for your migration: 
#    - run this file
#    - edit the migrations
#    - run the migrations
# 2) If you want to write the changes to the schema file:
#    - edit the schema file
#    - run this file
#    - check the migrations generated
#    - run the migrations

diesel migration generate --diff-schema $1