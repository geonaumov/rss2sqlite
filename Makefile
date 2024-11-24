# Makefile for Rust project

# Variables
CARGO := cargo
RUSTFMT := rustfmt
CLIPPY := clippy
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release

# Default target
.PHONY: all
all: format lint test build

# Format code using rustfmt
.PHONY: format
format:
	@echo "Formatting code..."
	$(CARGO) fmt --all
	@echo "Formatting complete!"

# Lint code using clippy
.PHONY: lint
lint:
	@echo "Linting code..."
	$(CARGO) clippy -- -D warnings
	@echo "Linting complete!"

# Run tests
.PHONY: test
test:
	@echo "Running tests..."
	$(CARGO) test
	@echo "Tests complete!"

# Build debug version
.PHONY: build
build:
	@echo "Building debug version..."
	$(CARGO) build
	@echo "Debug build complete!"

# Build release version
.PHONY: release
release:
	@echo "Building release version..."
	$(CARGO) build --release
	@echo "Release build complete!"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	@echo "Clean complete!"

# Check code without building
.PHONY: check
check:
	@echo "Checking code..."
	$(CARGO) check
	@echo "Check complete!"

# Run the program (debug version)
.PHONY: run
run:
	@echo "Running program..."
	$(CARGO) run

# Run the program (release version)
.PHONY: run-release
run-release:
	@echo "Running program (release)..."
	$(CARGO) run --release

# Update dependencies
.PHONY: update
update:
	@echo "Updating dependencies..."
	$(CARGO) update
	@echo "Update complete!"

# Generate documentation
.PHONY: doc
doc:
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps
	@echo "Documentation complete!"

# Watch for changes and run tests
.PHONY: watch
watch:
	@echo "Watching for changes..."
	$(CARGO) watch -x test

# Install cargo tools if needed
.PHONY: install-tools
install-tools:
	@echo "Installing cargo tools..."
	cargo install cargo-watch
	cargo install cargo-clippy
	rustup component add rustfmt
	rustup component add clippy
	@echo "Tools installation complete!"

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all          - Format, lint, test, and build"
	@echo "  format       - Format code using rustfmt"
	@echo "  lint         - Lint code using clippy"
	@echo "  test         - Run tests"
	@echo "  build        - Build debug version"
	@echo "  release      - Build release version"
	@echo "  clean        - Clean build artifacts"
	@echo "  check        - Check code without building"
	@echo "  run          - Run the program (debug)"
	@echo "  run-release  - Run the program (release)"
	@echo "  update       - Update dependencies"
	@echo "  doc          - Generate documentation"
	@echo "  watch        - Watch for changes and run tests"
	@echo "  install-tools- Install required cargo tools"
