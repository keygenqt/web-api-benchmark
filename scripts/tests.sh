#!/bin/bash

TESTS_DIR="reports"

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Get base path
API_BASE_URL_AXUM="http://host.docker.internal:3010"
API_BASE_URL_KTOR="http://host.docker.internal:3020"

# Pull docker k6
echo "## Add image tool k6 #########################"
docker pull grafana/k6:latest

# Builds dir
rm -rf "$TESTS_DIR"
mkdir "$TESTS_DIR"

# 30 seconds rest from the previous test
echo "## Get some rest cpu #########################"
sleep 30s

# Run axum tests
echo "## Run tests Axum... #########################"
docker run -e K6_LOG_OUTPUT=none --rm \
    --add-host host.docker.internal:host-gateway \
    -v $PWD/tests:/tests \
    -v $PWD/$TESTS_DIR:/$TESTS_DIR \
    grafana/k6 run --quiet \
    -e API_BASE_URL="$API_BASE_URL_AXUM" \
    -o web-dashboard=export="/$TESTS_DIR/report_tests_axum.html" \
    /tests/run_all.js

echo "## Done! Run (macOS): open -a 'Google Chrome' $PWD/$TESTS_DIR/report_tests_axum.html"

# 30 seconds rest from the previous test
echo "## Get some rest cpu #########################"
sleep 30s

# Run ktor tests
echo "## Run tests Ktor... #########################"
docker run -e K6_LOG_OUTPUT=none --rm \
    --add-host host.docker.internal:host-gateway \
    -v $PWD/tests:/tests \
    -v $PWD/$TESTS_DIR:/$TESTS_DIR \
    grafana/k6 run --quiet \
    -e API_BASE_URL="$API_BASE_URL_KTOR" \
    -o web-dashboard=export="/$TESTS_DIR/report_tests_ktor.html" \
    /tests/run_all.js

echo "## Done! Run (macOS): open -a 'Google Chrome' $PWD/$TESTS_DIR/report_tests_ktor.html"
