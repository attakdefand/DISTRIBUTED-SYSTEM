FROM rust:1.70 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml .
COPY services/order-service/Cargo.toml services/order-service/Cargo.toml

# This is a dummy build to get the dependencies cached
RUN mkdir -p services/order-service/src && echo "fn main() {}" > services/order-service/src/main.rs
RUN cargo build --release -p order-service

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release -p order-service

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/order-service ./order-service

# Expose port
EXPOSE 3002

# Run the application
CMD ["./order-service"]