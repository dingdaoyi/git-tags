FROM rust:1.73 as builder
WORKDIR /
RUN apt update && apt install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /target/x86_64-unknown-linux-musl/release/actix-web-app /usr/local/bin/
