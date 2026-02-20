#!/bin/bash

#################
## Run migrations
#################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Get postgres url
DATABASE_URL=$(bash scripts/migrations/database_url.sh)

# Run apply migrations
OUT=$(diesel migration run --database-url $DATABASE_URL 2>&1 | tee /dev/tty || exit)

# Done
if [[ -z "$OUT" ]]; then
    echo -e "\033[1;33mNo migrations found.\033[00m"
else
    echo -e "\033[1;32mMigration apply successfully.\033[00m"
fi
