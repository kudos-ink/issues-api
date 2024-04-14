DATABASE_URL=postgres://postgres:password@localhost:5432/database
DATABASE_INIT_FILE=db.sql
HTTP_SERVER_HOST=0.0.0.0
HTTP_SERVER_PORT=8000
USERNAME=test
PASSWORD=test
DOCKER_DB_CONTAINER_NAME:=db
DOCKER_COMPOSE:=docker-compose
DOCKER_COMPOSE_FILE:=docker-compose.yaml

# API

.PHONY: run
run:
	USERNAME="$(USERNAME)" PASSWORD="$(PASSWORD)" DATABASE_URL="$(DATABASE_URL)" HTTP_SERVER_HOST="$(HTTP_SERVER_HOST)" HTTP_SERVER_PORT=$(HTTP_SERVER_PORT) cargo run

.PHONY: test
test:
	cargo test

# DB

# Start the PostgreSQL container
.PHONY: db-up
db-up:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up $(DOCKER_DB_CONTAINER_NAME) -d

# Stop and remove the PostgreSQL container
.PHONY: db-down
db-down:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down $(DOCKER_DB_CONTAINER_NAME)

.PHONY: db-migrate
db-migrate:
	DATABASE_URL="$(DATABASE_URL)" diesel migration run

# Clean up the Docker volume
.PHONY: db-clean
db-clean:
	$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down $(DOCKER_DB_CONTAINER_NAME) -v
	
.PHONY: test-db
test-db:
	cargo test -- --ignored --test-threads=1
