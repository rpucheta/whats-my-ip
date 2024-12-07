# Variables
APP_NAME=WhatsMyIp
BINARY_NAME := whatsmyip
TARGET_DIR := target/release
SRC := src

# Default target: build release binary
.PHONY: all
all: build

# Build the release binary
.PHONY: build
build: ## Build the release binary
	@cargo build --release
	@echo "Build complete: $(TARGET_DIR)/$(BINARY_NAME)"

# Run the release binary
.PHONY: run
run: build ## Run the release binary
	$(TARGET_DIR)/$(BINARY_NAME)

# Clean build artifacts
.PHONY: clean
clean: ## Clean build artifacts
	@cargo clean
	@rm -rf lcov.info
	@rm -rf tarpaulin-report.html
	@echo "Clean complete."

# Test the project
.PHONY: test
test: ## Run tests
	@cargo test
	@echo "Tests complete."

# Run coverage using cargo-tarpaulin
tarpaulin: ## Run coverage using cargo-tarpaulin
	@cargo tarpaulin --out Html --out Lcov

# Format the code
.PHONY: format
format: ## Format the code
	@cargo fmt
	@echo "Code formatted."

# Lint the project
.PHONY: lint
lint: ## Lint the code
	@cargo clippy -- -D warnings
	@echo "Lint complete."

# Help menu
.PHONY: help
help: ## Show this help message
	@printf "\n\033[1m%s\033[0m\n" "$(APP_NAME)"
	@echo "Usage:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
