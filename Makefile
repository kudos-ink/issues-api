DATABASE_URL=postgres://postgres:password@localhost:5432/database
DATABASE_INIT_FILE=db.sql
HTTP_SERVER_HOST=0.0.0.0
HTTP_SERVER_PORT=8000

.PHONY: run
run:
	DATABASE_URL="$(DATABASE_URL)" DATABASE_INIT_FILE="$(DATABASE_INIT_FILE)" HTTP_SERVER_HOST="$(HTTP_SERVER_HOST)" HTTP_SERVER_PORT=$(HTTP_SERVER_PORT) cargo run


.PHONY: test
test:
	cargo test


.PHONY: test-db
test-db:
	cargo test -- --ignored --test-threads=1
