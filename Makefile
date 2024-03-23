DATABASE_URL=postgres://postgres:password@localhost:5432/database
TEST_DATABASE_URL=postgres://postgres:password@localhost:5433/database_test
DATABASE_INIT_FILE=db.sql
HTTP_SERVER_HOST=0.0.0.0
HTTP_SERVER_PORT=8000
PSEUDO=
PASSWORD=

.PHONY: run
run:
	PSEUDO="$(PSEUDO)" PASSWORD="$(PASSWORD)" DATABASE_URL="$(DATABASE_URL)" DATABASE_INIT_FILE="$(DATABASE_INIT_FILE)" HTTP_SERVER_HOST="$(HTTP_SERVER_HOST)" HTTP_SERVER_PORT=$(HTTP_SERVER_PORT) cargo run


.PHONY: db-up
db-up:
	docker compose up db

.PHONY: db-down
db-down:
	docker compose up down

.PHONY: db-migrate
db-migrate:
	DATABASE_URL="$(DATABASE_URL)" DATABASE_INIT_FILE="$(DATABASE_INIT_FILE)" cargo run

.PHONY: test-db-setup
test-db-setup:
	docker-compose up -d db-test

.PHONY: test-db-migrate
test-db-migrate:
	docker-compose exec -T db-test psql -U postgres -d database_test -f /$(DATABASE_INIT_FILE)

.PHONY: test-db-teardown
test-db-teardown:
	docker-compose down -v

.PHONY: test-db
test-db: test-db-setup test-db-migrate
	DATABASE_URL="$(TEST_DATABASE_URL)" PSEUDO="$(PSEUDO)" PASSWORD="$(PASSWORD)" cargo test -- --ignored --test-threads=1
	make test-db-teardown
