# Use Rust official image for building
FROM rust:1.75 as builder

# Create app directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal image for running
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-layer-system .

# Copy migrations
COPY --from=builder /app/migrations ./migrations

# Expose port (internal only, nginx handles external traffic)
EXPOSE 3000

# Run the application
CMD ["./rust-layer-system"]