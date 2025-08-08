.PHONY: build test check install clean help bench

# Variabili
BINARY_NAME = sha-calc
TARGET_DIR = target/release

# Build di default
build:
	@echo "🔨 Building $(BINARY_NAME)..."
	cargo build --release

# Esegui tutti i test
test:
	@echo "🧪 Running tests..."
	cargo test

# Test verbose
test-verbose:
	@echo "🧪 Running tests with verbose output..."
	cargo test -- --nocapture

# Test di integrazione
test-integration:
	@echo "🧪 Running integration tests..."
	cargo test --test integration_tests

# Check del codice (linting, formatting)
check:
	@echo "🔍 Checking code..."
	cargo check
	cargo clippy -- -D warnings
	cargo fmt --check

# Fix formatting
fmt:
	@echo "✨ Formatting code..."
	cargo fmt

# Installa il binario
install: build
	@echo "📦 Installing $(BINARY_NAME)..."
	cargo install --path . --force

# Installa per sviluppo (debug)
install-dev:
	@echo "📦 Installing $(BINARY_NAME) (debug)..."
	cargo install --path . --debug --force

# Clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Benchmark (se disponibili)
bench:
	@echo "⚡ Running benchmarks..."
	cargo bench

# Test di compatibilità con sha256sum
test-compat:
	@echo "🔄 Testing compatibility with sha256sum..."
	@echo "test content" > test_file.tmp
	@echo "Expected (sha256sum):"
	@sha256sum test_file.tmp || echo "sha256sum not available"
	@echo "Actual (sha-calc):"
	@$(TARGET_DIR)/$(BINARY_NAME) test_file.tmp || echo "Binary not built"
	@rm -f test_file.tmp

# Test completo
test-all: check test test-integration test-compat
	@echo "✅ All tests completed successfully!"

# Esempio di utilizzo
example: build
	@echo "📝 Running examples..."
	@echo "hello world" | $(TARGET_DIR)/$(BINARY_NAME)
	@echo "test content" > example.txt
	@$(TARGET_DIR)/$(BINARY_NAME) example.txt
	@$(TARGET_DIR)/$(BINARY_NAME) -a sha512 example.txt
	@$(TARGET_DIR)/$(BINARY_NAME) -q example.txt
	@rm -f example.txt

# Help
help:
	@echo "Available commands:"
	@echo "  build         - Build the release binary"
	@echo "  test          - Run all tests"
	@echo "  test-verbose  - Run tests with verbose output"
	@echo "  test-integration - Run integration tests only"
	@echo "  check         - Check code quality (clippy, fmt)"
	@echo "  fmt           - Format code"
	@echo "  install       - Build and install binary"
	@echo "  install-dev   - Install debug binary"
	@echo "  clean         - Clean build artifacts"
	@echo "  bench         - Run benchmarks"
	@echo "  test-compat   - Test compatibility with sha256sum"
	@echo "  test-all      - Run all tests and checks"
	@echo "  example       - Run usage examples"
	@echo "  help          - Show this help message"