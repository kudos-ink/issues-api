DATABASE_URL?=postgres://postgres:password@localhost:5432/database
HOST?=0.0.0.0
PORT?=8000
USERNAME?=test
PASSWORD?=test
DOCKER_DB_CONTAINER_NAME:=db
DOCKER_COMPOSE:=docker compose
DOCKER_API_CONTAINER_NAME:=api
DOCKER_COMPOSE_FILE:=docker-compose.yaml

# API

.PHONY: run
run:
	USERNAME="$(USERNAME)" PASSWORD="$(PASSWORD)" DATABASE_URL="$(DATABASE_URL)" HOST="$(HOST)" PORT=$(PORT) cargo run

.PHONY: test
test:
	DATABASE_URL="$(DATABASE_URL)" cargo test

# Start the PostgreSQL container
.PHONY: docker-api
docker-api:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up $(DOCKER_API_CONTAINER_NAME) 
# DB

# Start the PostgreSQL container
.PHONY: db-up
db-up:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up $(DOCKER_DB_CONTAINER_NAME) -d

# Stop and remove the PostgreSQL container
.PHONY: db-down
db-down:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down $(DOCKER_DB_CONTAINER_NAME)

.PHONY: db-migrate-up
db-migrate-up:
	DATABASE_URL="$(DATABASE_URL)" diesel migration run

.PHONY: db-migrate-down
db-migrate-down:
	DATABASE_URL="$(DATABASE_URL)" diesel migration revert

.PHONY: db-migrate-redo
db-migrate-redo:
	DATABASE_URL="$(DATABASE_URL)" diesel migration redo

# Clean up the Docker volume
.PHONY: db-clean
db-clean:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down $(DOCKER_DB_CONTAINER_NAME) -v
	
.PHONY: test-db
test-db:
	DATABASE_URL="$(DATABASE_URL)" cargo test -- --ignored --test-threads=1