#!/bin/zsh
cd "$(dirname "$0")"

docker compose --env-file ../../.env up -d