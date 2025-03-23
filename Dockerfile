# Step 1: Build Stage
FROM rust:1.85-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq-dev pkg-config build-essential

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm -rf target/release/deps/Spenser*

COPY . .
RUN cargo build --release

# Step 2: Production Stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq5 openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/Spenser .

EXPOSE 8005

CMD ["./Spenser"]
