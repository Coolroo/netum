#!/bin/bash

cd "$(dirname "$0")"

docker build -t netum/backend ../netum_api

cd ../docker

docker compose up