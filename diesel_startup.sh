#!/bin/bash

echo "diesel_startup starting"

if [ -z ${DATABASE_URL+x} ]; then
  echo "DATABASE_URL is unset"
else
  echo "DATABASE_URL is set to '$DATABASE_URL'"
fi

# Define a flag to track whether the migration has succeeded
migration_succeeded=false

# Loop until the migration succeeds
while [ "$migration_succeeded" != true ]; do
  echo "Running diesel migration..."
  diesel migration run --migration-dir=/usr/src/portfolio_api/migrations
  
  # Check the exit status of the last command
  if [ $? -eq 0 ]; then
    echo "Migration succeeded"
    migration_succeeded=true
  else
    echo "Migration failed. Retrying in 2 seconds..."
    sleep 2
  fi
done

echo "diesel migration finished"
