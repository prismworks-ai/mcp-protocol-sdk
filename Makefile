# Makefile for MCP Protocol SDK Development

.PHONY: help setup check quick-check format lint test test-all examples docs clean audit coverage bench

# Default target
help: ## Show this help message
	@echo "MCP Protocol SDK - Development Commands"
	@echo "==================================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Examples:"
	@echo "  make setup      # Set up development environment"
	@echo "  make check      # Run all CI checks locally"
	@echo "  make quick      # Run essential checks only"
	@echo "  make test       # Run all tests"

setup: ## Install development tools and dependencies
	@echo "🔧 Setting up development environment..."
	./scripts/setup-dev.sh

check: ## Run full CI checks (mirrors GitHub Actions)
	@echo "🚀 Running full CI checks..."
	./scripts/ci-check.sh

quick-check: ## Run essential checks only (faster)
	@echo "⚡ Running quick checks..."
	./scripts/ci-check.sh --quick

format: ## Format code with rustfmt
	@echo "🎨 Formatting code..."
	cargo fmt --all

format-check: ## Check if code is properly formatted
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check

lint: ## Run clippy lints
	@echo "🧹 Running clippy lints..."
	cargo clippy --all-features -- -W clippy::all -A unused_imports -A unused_variables -A dead_code -A unused_mut -A private_interfaces -A clippy::redundant_closure -A clippy::redundant_pattern_matching -A clippy::should_implement_trait -A clippy::manual_strip -A clippy::type_complexity

lint-fix: ## Automatically fix clippy issues
	@echo "🔧 Fixing clippy issues..."
	cargo clippy --all-features --fix

compile: ## Check compilation
	@echo "🔨 Checking compilation..."
	cargo check --all-features

test: ## Run tests with default features
	@echo "🧪 Running tests..."
	cargo test --verbose

test-all: ## Run tests with all feature combinations
	@echo "🧪 Running all tests..."
	cargo test --all-features --verbose
	cargo test --no-default-features --verbose
	cargo test --features stdio --verbose
	cargo test --features http --verbose
	cargo test --features websocket --verbose
	cargo test --features validation --verbose

examples: ## Check that all examples compile
	@echo "📚 Checking examples..."
	cargo check --example simple_server
	cargo check --example echo_server
	cargo check --example client_example
	cargo check --example database_server
	cargo check --example http_server --features http
	cargo check --example http_client --features http
	cargo check --example websocket_server --features websocket
	cargo check --example websocket_client --features websocket

docs: ## Build documentation
	@echo "📖 Building documentation..."
	cargo doc --all-features --no-deps --document-private-items

docs-open: ## Build and open documentation
	@echo "📖 Building and opening documentation..."
	cargo doc --all-features --no-deps --document-private-items --open

clean: ## Clean build artifacts
	@echo "🧽 Cleaning build artifacts..."
	cargo clean

audit: ## Run security audit
	@echo "🔒 Running security audit..."
	cargo audit

coverage: ## Generate code coverage report
	@echo "📊 Generating code coverage..."
	cargo tarpaulin --all-features --workspace --timeout 120 --out html
	@echo "Coverage report generated: tarpaulin-report.html"

bench: ## Run benchmarks
	@echo "🏎️  Running benchmarks..."
	cargo bench

# Development workflow targets
dev-setup: setup ## Complete development setup
	@echo "✅ Development environment ready!"

pre-commit: format lint compile test examples ## Run pre-commit checks
	@echo "✅ Pre-commit checks passed!"

ci-local: check ## Run full CI checks locally
	@echo "✅ Local CI checks complete!"

# Release preparation targets
pre-release: test-all docs audit ## Prepare for release
	@echo "🚀 Ready for release!"

# Feature-specific targets
test-stdio: ## Test STDIO features only
	cargo test --features stdio --verbose

test-http: ## Test HTTP features only
	cargo test --features http --verbose

test-websocket: ## Test WebSocket features only
	cargo test --features websocket --verbose

test-validation: ## Test validation features only
	cargo test --features validation --verbose

# Utility targets
watch: ## Watch for changes and run tests
	@echo "👀 Watching for changes..."
	cargo watch -x "test --all-features"

watch-check: ## Watch for changes and run checks
	@echo "👀 Watching for changes (checks only)..."
	cargo watch -x "check --all-features"

deps: ## Show dependency tree
	cargo tree --all-features

outdated: ## Check for outdated dependencies
	cargo outdated

update: ## Update dependencies
	cargo update

# Docker targets (if needed)
docker-build: ## Build Docker image for testing
	@echo "🐳 Building Docker image..."
	docker build -t mcp-protocol-sdk .

docker-test: ## Run tests in Docker
	@echo "🐳 Running tests in Docker..."
	docker run --rm mcp-protocol-sdk cargo test --all-features

# Git hooks
install-hooks: ## Install git hooks
	@echo "🪝 Installing git hooks..."
	cp .git/hooks/pre-commit.sample .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

# Size analysis
bloat: ## Analyze binary size
	@echo "📏 Analyzing binary size..."
	cargo bloat --release --crates

# Cross-compilation checks
check-cross: ## Check cross-compilation targets
	@echo "🌍 Checking cross-compilation..."
	cargo check --target x86_64-unknown-linux-gnu
	cargo check --target x86_64-pc-windows-gnu
	cargo check --target x86_64-apple-darwin
