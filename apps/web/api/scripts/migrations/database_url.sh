#!/bin/bash

###########################
## Get url connect database
###########################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# Init config
bash scripts/init.sh >/dev/null 2>&1

# Get params
CONF_POSTGRES=$(sed -n '/postgres {/,/}/p' ../docker/configs/api.conf)
PASSWORD=$(echo "$CONF_POSTGRES" | grep 'password = ' | sed -E 's/.*"([^"]+)".*/\1/')
USER=$(echo "$CONF_POSTGRES" | grep 'user = ' | sed -E 's/.*"([^"]+)".*/\1/')
URL=$(echo "$CONF_POSTGRES" | grep 'url = ' | sed -E 's/.*"([^"]+)".*/\1/')

# Create link
HOST=${URL#postgres://}
DATABASE_URL="postgres://$USER:$PASSWORD@$HOST"

# Done
echo -e $DATABASE_URL
