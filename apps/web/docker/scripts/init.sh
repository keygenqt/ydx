#!/bin/bash

##############################
## Init configuration services
##############################

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../";

# Config application
if [[ ! -e ".env" ]]; then
    cp .env.example .env
fi

# Load .env
export $(grep -v '^#' .env | xargs)

# Create dir
PATH_CONFIGS="configs"
mkdir -p "$PATH_CONFIGS"

# Config application
cat > "$PATH_CONFIGS/api.conf" <<EOF
server {
    development = false
    secret = ${API_SECRET}
}
deployment {
    host = 0.0.0.0
    port = $API_PORT
    workdir = "/var/www/api"
}
postgres {
    url = "postgres://postgres:5432/${POSTGRES_DB}"
    user = "${POSTGRES_USER}"
    password = "${POSTGRES_PASSWORD}"
}
EOF

# Create connect config pgadmin
cat > "$PATH_CONFIGS/pgadmin.json" <<EOF
{
    "Servers": {
    "1": {
        "Name": "My Local PostgresSQL Database",
        "Group": "Servers",
        "Host": "postgres",
        "Port": 5432,
        "MaintenanceDB": "${POSTGRES_DB}",
        "Username": "${POSTGRES_USER}",
        "SSLMode": "prefer",
        "Comment": "Automatically connected server via Docker Compose."
    }
    }
}
EOF

# Create nginx proxy
cat > "$PATH_CONFIGS/nginx.conf" <<EOF
server {
    listen 0.0.0.0:$APP_PORT;

    charset UTF-8;

    root /var/www/app/dist;
    index index.html;

    location /docs {
        alias /var/www/docs/book/;
        try_files \$uri \$uri/ /docs/index.html;
    }

    location /api {
        proxy_pass http://api:$API_PORT;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }

    location /pgadmin {
        proxy_pass http://pgadmin;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }

    location /uploads {
        alias /var/www/api/uploads;
        expires 30d;
        add_header Cache-Control "public, no-transform";
        location ~ ^/uploads/$ {
            return 403;
        }
    }

    location / {
        try_files \$uri /index.html;
    }
}
EOF

# Done
echo -e "\033[1;32mInit successfully.\033[00m"
