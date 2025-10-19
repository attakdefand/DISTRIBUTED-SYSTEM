FROM rust:1.70 as builder

WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml .
COPY bff/web-bff/Cargo.toml bff/web-bff/Cargo.toml

# This is a dummy build to get the dependencies cached
RUN mkdir -p bff/web-bff/src && echo "fn main() {}" > bff/web-bff/src/main.rs
RUN cargo build --release -p web-bff

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release -p web-bff

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/web-bff ./web-bff

# Expose port
EXPOSE 3003

# Run the application
CMD ["./web-bff"]