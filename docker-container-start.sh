#!/usr/bin/env bash

# for use within the docker container

set -e

if [ ! -f "db/mealplanner.db" ]; then
    echo No db found, creating one
    touch db/mealplanner.db
fi

DATABASE_URL="sqlite:db/mealplanner.db?mode=rwc" ./migration up

./server
