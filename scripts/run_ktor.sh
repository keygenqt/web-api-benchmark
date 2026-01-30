#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Build axum
source scripts/build_ktor.sh

# Run server
cd $PWD/$PROJECTS_DIR/$PROJECT_KTOR
java -jar "$PROJECT_KTOR.jar" start --config application.conf
