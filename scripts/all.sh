#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

source scripts/up.sh
source scripts/tests.sh