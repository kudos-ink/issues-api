#!/bin/bash

BASE_URL=${BASE_URL:-"http://localhost:8000"}
AUTH_HEADER="Authorization: Basic ${AUTH_TOKEN}"
CONTENT_TYPE_HEADER="Content-Type: application/json"

# Create Polkadot project
curl --location "$BASE_URL/projects" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "name": "polkadot",
    "slug":"polkadot"
}'

# Create Asar project
curl --location "$BASE_URL/projects" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "name": "astar",
    "slug":"astar"
}'
# Create Polkadot SDK repository
curl --location "$BASE_URL/repositories" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "name": "Polkadot SDK",
    "slug": "polkadotsdk",
    "language_slug": "rust",
    "url": "https://github.com/paritytech/polkadot-sdk",
    "project_id": 1
}'

# Create Zombienet repository
curl --location "$BASE_URL/repositories" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "name": "Zombienet",
    "slug": "zombienet",
    "language_slug": "rust",
    "url": "https://github.com/paritytech/zombienet",
    "project_id": 1
}'

# Create Astar repository
curl --location "$BASE_URL/repositories" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "name": "Astar",
    "slug": "astar",
    "language_slug": "rust",
    "url": "https://github.com/AstarNetwork/Astar",
    "project_id": 2
}'

# Create issues
curl --location "$BASE_URL/issues" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "number": 1863,
    "title": "Adding separate label to the zombie namespaces",
    "open": true,
    "certified": false,
    "repository_id": 1,
    "issue_created_at": "2024-09-03T09:13:54Z"
}'

curl --location "$BASE_URL/issues" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "number": 5597,
    "title": "Fix balance to u256 type",
    "open": true,
    "certified": false,
    "repository_id": 2,
    "issue_created_at": "2024-09-03T09:13:54Z"
}'

curl --location "$BASE_URL/issues" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "number": 1350,
    "title": "Update mocks to use TestDefaultConfig",
    "open": true,
    "certified": false,
    "repository_id": 3,
    "issue_created_at": "2024-09-03T09:13:54Z"
}'

# Cretae users

curl --location "$BASE_URL/users" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "username": "leapalazzolo"
}'

curl --location "$BASE_URL/users" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "username": "CJ13th"
}'

curl --location "$BASE_URL/users" \
--header "$AUTH_HEADER" \
--header "$CONTENT_TYPE_HEADER" \
--data '{
    "username": "ipapandinas"
}'