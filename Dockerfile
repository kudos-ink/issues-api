FROM --platform=linux/arm64/v8 rust:alpine as builder

RUN apk add --no-cache musl-dev
RUN USER=root cargo new --bin kudos_api
WORKDIR /kudos_api

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM --platform=linux/arm64/v8 alpine:latest
COPY --from=builder /kudos_api/target/release/kudos_api /usr/local/bin/

CMD ["kudos_api"]
