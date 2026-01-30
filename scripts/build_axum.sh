#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Load config
source scripts/config.sh

# Builds dir
rm -rf "$PROJECTS_DIR/$PROJECT_AXUM"
mkdir -p "$PROJECTS_DIR/$PROJECT_AXUM"

# Build axum
cargo build --release --manifest-path "$PWD/$PROJECT_AXUM/Cargo.toml"

# Copy
cp "$PWD/$PROJECT_AXUM/target/release/$PROJECT_AXUM" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM"
cp -r "$PWD/$PROJECT_AXUM/migrations" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM"
cp "$PWD/$PROJECT_AXUM/application.conf" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM"

# Up config
sed -i "s/my_dbname/$POSTGRES_DB/g" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM/application.conf"
sed -i "s/username/$POSTGRES_USER/g" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM/application.conf"
sed -i "s/00000000/$POSTGRES_PASSWORD/g" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM/application.conf"
sed -i "s/3000/$PROJECT_AXUM_PORT/g" "$PWD/$PROJECTS_DIR/$PROJECT_AXUM/application.conf"
