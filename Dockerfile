# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.74.0
FROM rust:${RUST_VERSION} AS build

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/kudos_api /bin/kudos_api
EOF

FROM debian:bookworm-slim AS final

ARG SERVER_PORT=8000
ARG UID=10001

# Install libpq and other necessary libraries
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       libpq5 \
    && rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

COPY --from=build /bin/kudos_api /bin/

EXPOSE ${SERVER_PORT}

CMD ["kudos_api"]
