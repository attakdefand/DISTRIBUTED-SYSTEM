FROM rust:1.70 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml .
COPY services/user-service/Cargo.toml services/user-service/Cargo.toml

# This is a dummy build to get the dependencies cached
RUN mkdir -p services/user-service/src && echo "fn main() {}" > services/user-service/src/main.rs
RUN cargo build --release -p user-service

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release -p user-service

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/user-service ./user-service

# Expose port
EXPOSE 3001

# Run the application
CMD ["./user-service"]