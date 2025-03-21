# Step 1: Build Stage
FROM rust:1.85-slim AS builder

WORKDIR /app

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev pkg-config build-essential

# Copy Cargo manifests first (for caching dependencies)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies separately to leverage Docker caching
RUN cargo build --release
RUN rm -rf target/release/deps/Spenser*

# Copy actual application code
COPY . .
RUN cargo build --release

# Step 2: Production Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install only the necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 openssl && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/Spenser .
COPY --from=builder /app/migrations ./migrations

# Expose the application port
EXPOSE 8005

# Start the application
CMD ["./Spenser"]
