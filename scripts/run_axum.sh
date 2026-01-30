#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Build axum
source scripts/build_axum.sh

# Run server
cd $PWD/$PROJECTS_DIR/$PROJECT_AXUM
./$PROJECT_AXUM start --config application.conf
