#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Remove docker containers
echo "## Remove containers #########################"
docker compose -f kotlin-ktor/docker/compose.yml down
docker compose -f rust-axum/docker/compose.yml down

# Create docker containers
echo "## Create containers #########################"
docker compose -f kotlin-ktor/docker/compose.yml up --no-build -d
docker compose -f rust-axum/docker/compose.yml up --no-build -d
