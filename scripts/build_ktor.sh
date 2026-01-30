#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Load config
source scripts/config.sh

# Builds dir
rm -rf "$PROJECTS_DIR/$PROJECT_KTOR"
mkdir -p "$PROJECTS_DIR/$PROJECT_KTOR"

# Build ktor
$PROJECT_KTOR/gradlew buildFatJar --project-dir "$PROJECT_KTOR"

# Copy
cp "$PWD/$PROJECT_KTOR/build/libs/$PROJECT_KTOR.jar" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR"
cp -r "$PWD/$PROJECT_KTOR/migrations" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR"
cp "$PWD/$PROJECT_KTOR/application.conf" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR"

# Up config
sed -i "s/my_dbname/$POSTGRES_DB/g" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR/application.conf"
sed -i "s/username/$POSTGRES_USER/g" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR/application.conf"
sed -i "s/00000000/$POSTGRES_PASSWORD/g" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR/application.conf"
sed -i "s/3000/$PROJECT_KTOR_PORT/g" "$PWD/$PROJECTS_DIR/$PROJECT_KTOR/application.conf"
