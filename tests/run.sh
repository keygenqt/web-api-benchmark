#!/bin/bash

TESTS_DIR="TESTS"

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Load config
source scripts/config.sh

# Builds dir
rm -rf "$TESTS_DIR"
mkdir "$TESTS_DIR"

# Get base path
API_BASE_URL_AXUM="http://0.0.0.0:$PROJECT_AXUM_PORT"
API_BASE_URL_KTOR="http://0.0.0.0:$PROJECT_KTOR_PORT"

# Run ktor tests
echo "############## Ktor ##############"

K6_LOG_OUTPUT=none k6 run -e API_BASE_URL="$API_BASE_URL_KTOR" \
    --out web-dashboard=export="$TESTS_DIR/ktor_run_all.html" \
    tests/run_all.js

# 5 seconds rest from the previous test
echo
echo ">> Let's give it a rest"
echo

sleep 10s

# Run axum tests
echo "############## Axum ##############"

K6_LOG_OUTPUT=none k6 run -e API_BASE_URL="$API_BASE_URL_AXUM" \
    --out web-dashboard=export="$TESTS_DIR/axum_run_all.html" \
    tests/run_all.js
