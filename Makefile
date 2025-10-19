# Makefile for Microservices Architecture

# Default target
.PHONY: help
help:
	@echo "Microservices Architecture Makefile"
	@echo "Available targets:"
	@echo "  build            - Build all services"
	@echo "  test             - Run all tests"
	@echo "  test-unit        - Run unit tests"
	@echo "  test-integration - Run integration tests"
	@echo "  test-patterns    - Run pattern verification tests"
	@echo "  run              - Run all services with docker-compose"
	@echo "  clean            - Clean target directory"
	@echo "  docs             - Generate documentation"

# Build all services
.PHONY: build
build:
	cargo build

# Run all tests
.PHONY: test
test: test-unit test-integration test-patterns

# Run unit tests
.PHONY: test-unit
test-unit:
	cargo test --lib

# Run integration tests
.PHONY: test-integration
test-integration:
	cargo test --test integration_tests

# Run pattern verification tests
.PHONY: test-patterns
test-patterns:
	cargo test --test pattern_tests

# Run all services with docker-compose
.PHONY: run
run:
	docker-compose up --build

# Clean target directory
.PHONY: clean
clean:
	cargo clean

# Generate documentation
.PHONY: docs
docs:
	cargo doc --workspace --no-deps