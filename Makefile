# four-code Makefile
# Rust equivalent workflow to PHP (PHPStan, PSR, PHPUnit)

.PHONY: all check test lint fmt clean build release run

# Default: run all checks (like CI)
all: fmt lint test

# Quick syntax check (fastest)
check:
	cargo check --all-targets

# Run all tests (like PHPUnit)
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Lint with clippy (like PHPStan level max)
lint:
	cargo clippy --all-targets -- -D warnings

# Format code (like PSR-12 / php-cs-fixer)
fmt:
	cargo fmt --all

# Check formatting without changing (for CI)
fmt-check:
	cargo fmt --all -- --check

# Build debug
build:
	cargo build

# Build release (optimized)
release:
	cargo build --release
	@echo "Binary: target/release/four-code"
	@ls -lh target/release/four-code

# Run the editor
run:
	cargo run

# Run with a file
run-file:
	cargo run -- $(FILE)

# Clean build artifacts
clean:
	cargo clean

# Full CI check (what CI should run)
ci: fmt-check lint test
	@echo "All CI checks passed!"

# Watch mode (requires cargo-watch: cargo install cargo-watch)
watch:
	cargo watch -x check -x test

# Generate documentation
doc:
	cargo doc --no-deps --open

# Show binary size
size: release
	@echo "\nBinary size breakdown:"
	@size target/release/four-code || true
	@echo "\nStripped size:"
	@strip -s target/release/four-code -o /tmp/four-code-stripped
	@ls -lh /tmp/four-code-stripped

# Help
help:
	@echo "four-code build commands:"
	@echo ""
	@echo "  make          - Run fmt, lint, test (full check)"
	@echo "  make check    - Quick syntax check"
	@echo "  make test     - Run all tests"
	@echo "  make lint     - Run clippy linter"
	@echo "  make fmt      - Format code"
	@echo "  make build    - Debug build"
	@echo "  make release  - Release build"
	@echo "  make run      - Run editor"
	@echo "  make ci       - Full CI check"
	@echo "  make clean    - Clean build"
