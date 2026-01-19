FROM rust:1.92 AS builder
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /target/release/axum_http_server /usr/local/bin/axum_http_server
EXPOSE 8000
CMD ["axum_http_server"]
