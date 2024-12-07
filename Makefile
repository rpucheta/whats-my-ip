# Variables
BINARY_NAME := whatsmyip
TARGET_DIR := target/release
SRC := src

# Default target: build release binary
.PHONY: all
all: build

# Build the release binary
.PHONY: build
build:
	@cargo build --release
	@echo "Build complete: $(TARGET_DIR)/$(BINARY_NAME)"

# Run the release binary
.PHONY: run
run: build
	$(TARGET_DIR)/$(BINARY_NAME)

# Clean build artifacts
.PHONY: clean
clean:
	@cargo clean
	@rm -rf lcov.info
	@rm -rf tarpaulin-report.html
	@echo "Clean complete."

# Test the project
.PHONY: test
test:
	@cargo test
	@echo "Tests complete."

# Run coverage using cargo-tarpaulin
tarpaulin:
	@cargo tarpaulin --out Html --out Lcov

# Format the code
.PHONY: format
format:
	@cargo fmt
	@echo "Code formatted."

# Lint the project
.PHONY: lint
lint:
	@cargo clippy -- -D warnings
	@echo "Lint complete."

# Help menu
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make build    - Build the release binary"
	@echo "  make run      - Run the release binary"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make test     - Run tests"
	@echo "  make tarpaulin - Run coverage using cargo-tarpaulin"
	@echo "  make format   - Format the code"
	@echo "  make lint     - Lint the code"
	@echo "  make help     - Show this help message"