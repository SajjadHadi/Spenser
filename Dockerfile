FROM rust:1.85-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev libpq-dev

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf target/release/deps/Spenser*

COPY . .

RUN cargo build --release

FROM alpine:3.21

WORKDIR /app

RUN apk add --no-cache libpq openssl

COPY --from=builder /app/target/release/Spenser .
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./Spenser"]
