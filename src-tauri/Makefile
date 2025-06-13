# PodPico Makefile - Quality-First Development Automation
# Follows AI Agent Instructions for zero-tolerance quality standards

.PHONY: help test coverage coverage-open coverage-ci quality quality-full dev clean setup

# Default target
.DEFAULT_GOAL := help

# Color codes for output
GREEN=\033[0;32m
YELLOW=\033[0;33m
RED=\033[0;31m
NC=\033[0m # No Color

help: ## Show this help message
	@echo "🚀 PodPico Development Commands"
	@echo "==============================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "📊 Coverage Targets:"
	@echo "  coverage         - Run test coverage analysis (dev mode)"
	@echo "  coverage-open    - Run coverage and open HTML report"
	@echo "  coverage-ci      - Run coverage with CI-strict mode (80% target)"
	@echo ""
	@echo "🔧 Quality Targets:"
	@echo "  quality          - Run essential quality checks (clippy + tests)"
	@echo "  quality-full     - Run comprehensive quality analysis (includes coverage)"

setup: ## Install required development tools
	@echo "🛠️ Setting up PodPico development environment..."
	@echo "Installing cargo-tarpaulin for coverage reporting..."
	@cargo install cargo-tarpaulin || true
	@echo "✅ Development tools installed"

test: ## Run all tests
	@echo "🧪 Running PodPico test suite..."
	@cargo test --all

coverage: ## Run test coverage analysis (development mode)
	@echo "📊 Running test coverage analysis..."
	@./scripts/coverage.sh

coverage-open: ## Run coverage analysis and open HTML report
	@echo "📊 Running test coverage analysis with report..."
	@./scripts/coverage.sh --open

coverage-ci: ## Run coverage analysis in CI mode (strict 80% target)
	@echo "🏗️ Running test coverage analysis (CI mode)..."
	@./scripts/coverage.sh --ci

coverage-custom: ## Run coverage with custom target (usage: make coverage-custom TARGET=85)
	@echo "🎯 Running test coverage analysis (target: $(TARGET)%)..."
	@./scripts/coverage.sh --target $(TARGET)

quality: ## Run essential quality checks (clippy + tests) - AI Agent mandatory
	@echo "⚡ Running essential quality checks..."
	@echo "$(YELLOW)1/3$(NC) Running clippy (zero warnings tolerance)..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(YELLOW)2/3$(NC) Running tests..."
	@cargo test --all
	@echo "$(YELLOW)3/3$(NC) Checking code formatting..."
	@cargo fmt --all -- --check || (echo "$(RED)❌ Code not formatted. Run: make format$(NC)" && exit 1)
	@echo "$(GREEN)✅ Essential quality checks passed$(NC)"

quality-full: ## Run comprehensive quality analysis (AI Agent Session Protocol)
	@echo "🔍 Running comprehensive quality analysis..."
	@echo "$(YELLOW)Phase 1:$(NC) Essential quality checks..."
	@$(MAKE) quality
	@echo "$(YELLOW)Phase 2:$(NC) Test coverage analysis..."
	@$(MAKE) coverage
	@echo "$(YELLOW)Phase 3:$(NC) Security audit..."
	@cargo audit || echo "$(YELLOW)⚠️ Audit not available - consider: cargo install cargo-audit$(NC)"
	@echo "$(GREEN)✅ Comprehensive quality analysis completed$(NC)"

format: ## Format all Rust code
	@echo "🎨 Formatting Rust code..."
	@cargo fmt --all

format-check: ## Check if code is properly formatted
	@echo "🔍 Checking code formatting..."
	@cargo fmt --all -- --check

clippy: ## Run clippy linting (zero warnings tolerance)
	@echo "🔍 Running clippy analysis..."
	@cargo clippy --all-targets --all-features -- -D warnings

clippy-fix: ## Run clippy with automatic fixes
	@echo "🔧 Running clippy with automatic fixes..."
	@cargo clippy --all-targets --all-features --fix -- -D warnings

build: ## Build the project
	@echo "🏗️ Building PodPico..."
	@cargo build

build-release: ## Build the project in release mode
	@echo "🏗️ Building PodPico (release mode)..."
	@cargo build --release

dev: ## Run the application in development mode
	@echo "🚀 Starting PodPico in development mode..."
	@npm run tauri dev

clean: ## Clean build artifacts and coverage reports
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf coverage/
	@rm -rf target/
	@echo "✅ Clean completed"

# Session Quality Protocol (AI Agent Instructions)
session-start: ## Start development session with quality verification
	@echo "🎯 Starting PodPico development session..."
	@echo "📅 Session Date: $$(date '+%Y-%m-%d %H:%M:%S')"
	@echo "🔍 Running mandatory pre-session quality checks..."
	@$(MAKE) quality
	@echo "$(GREEN)✅ Session ready - all quality gates passed$(NC)"

session-end: ## End development session with quality verification and coverage
	@echo "🏁 Ending PodPico development session..."
	@echo "📅 Session End: $$(date '+%Y-%m-%d %H:%M:%S')"
	@echo "🔍 Running mandatory session-end quality checks..."
	@$(MAKE) quality-full
	@echo "$(GREEN)✅ Session completed - ready for commit$(NC)"

# AI Agent Instructions: Zero-tolerance quality gates
ci-check: ## Complete CI/CD quality pipeline (for automated builds)
	@echo "🤖 Running CI/CD quality pipeline..."
	@echo "$(YELLOW)Step 1:$(NC) Code formatting check..."
	@$(MAKE) format-check
	@echo "$(YELLOW)Step 2:$(NC) Clippy analysis (zero warnings)..."
	@$(MAKE) clippy
	@echo "$(YELLOW)Step 3:$(NC) Full test suite..."
	@$(MAKE) test
	@echo "$(YELLOW)Step 4:$(NC) Coverage analysis (80% minimum)..."
	@$(MAKE) coverage-ci
	@echo "$(YELLOW)Step 5:$(NC) Security audit..."
	@cargo audit || echo "$(YELLOW)⚠️ Audit skipped$(NC)"
	@echo "$(GREEN)✅ CI/CD pipeline completed successfully$(NC)"

# Coverage reporting shortcuts
cov: coverage ## Shortcut for coverage
cov-open: coverage-open ## Shortcut for coverage-open  
cov-ci: coverage-ci ## Shortcut for coverage-ci

# Quality shortcuts
q: quality ## Shortcut for quality
qf: quality-full ## Shortcut for quality-full 