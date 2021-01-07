FROM rust:alpine as builder
RUN apk add musl-dev
RUN rustup toolchain install nightly
RUN rustup target add --toolchain nightly x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
RUN cargo +nightly install --path api

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/api_bin /usr/local/bin/api_bin
ENTRYPOINT ["api_bin"]
