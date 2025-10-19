FROM rust:1.70 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml .
COPY gateway/Cargo.toml gateway/Cargo.toml

# This is a dummy build to get the dependencies cached
RUN mkdir -p gateway/src && echo "fn main() {}" > gateway/src/main.rs
RUN cargo build --release -p gateway

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release -p gateway

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/gateway ./gateway

# Expose port
EXPOSE 3000

# Run the application
CMD ["./gateway"]