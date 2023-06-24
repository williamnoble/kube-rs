FROM rust:1.62.0-alpine AS builder
RUN apk add musl-dev
WORKDIR /usr/src/
RUN USER=root cargo new hello-world-rs-operator
WORKDIR /usr/src/hello-world-rs-operator
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# Bundle Stage
FROM alpine as final
COPY --from=builder /usr/src/hello-world-rs-operator/target/release/hello-world-rs-operator .
CMD ["./hello-world-rs-operator"]