#!/bin/bash

# PodPico Test Coverage Automation Script
# Usage: ./scripts/coverage.sh [--ci] [--open]

set -e  # Exit on any error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
COVERAGE_DIR="$PROJECT_DIR/coverage"

# Default settings
CI_MODE=false
OPEN_REPORT=false
COVERAGE_TARGET=80

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --ci)
            CI_MODE=true
            shift
            ;;
        --open)
            OPEN_REPORT=true
            shift
            ;;
        --target)
            COVERAGE_TARGET="$2"
            shift 2
            ;;
        *)
            echo "Usage: $0 [--ci] [--open] [--target PERCENTAGE]"
            echo "  --ci        Run in CI mode (XML output, strict failure)"
            echo "  --open      Open HTML report after generation"
            echo "  --target N  Set coverage target percentage (default: 80)"
            exit 1
            ;;
    esac
done

# Get current date/time for session tracking
SESSION_DATE=$(date +"%Y-%m-%d")
SESSION_TIME=$(date +"%H:%M:%S")
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")

echo "🔍 PodPico Test Coverage Analysis - $TIMESTAMP"
echo "================================================"

# Navigate to project directory
cd "$PROJECT_DIR"

# Clean previous coverage data
echo "🧹 Cleaning previous coverage data..."
rm -rf "$COVERAGE_DIR"
mkdir -p "$COVERAGE_DIR"

# Ensure cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "❌ cargo-tarpaulin not found. Installing..."
    cargo install cargo-tarpaulin
fi

# Pre-coverage quality checks (mandatory from AI agent instructions)
echo "⚡ Running pre-coverage quality checks..."

# 1. Clippy check (zero warnings tolerance)
echo "  ├─ Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy check failed. Fix all warnings before running coverage."
    exit 1
fi

# 2. Format check
echo "  ├─ Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed. Run 'cargo fmt --all' first."
    exit 1
fi

# 3. Basic compilation check
echo "  ├─ Verifying compilation..."
if ! cargo check --all-targets --all-features; then
    echo "❌ Compilation check failed."
    exit 1
fi

echo "✅ Pre-coverage quality checks passed"

# Run coverage based on mode
if [ "$CI_MODE" = true ]; then
    echo "🏗️  Running coverage in CI mode..."
    TARPAULIN_CONFIG="ci"
    FAIL_UNDER_ARG="--fail-under $COVERAGE_TARGET"
else
    echo "🏗️  Running coverage in development mode..."
    TARPAULIN_CONFIG="podpico"
    FAIL_UNDER_ARG=""
fi

# Execute coverage analysis
echo "🧪 Analyzing test coverage..."
COVERAGE_START_TIME=$(date +%s)

if ! cargo tarpaulin --config "$TARPAULIN_CONFIG" $FAIL_UNDER_ARG; then
    COVERAGE_EXIT_CODE=$?
    echo "❌ Coverage analysis failed with exit code $COVERAGE_EXIT_CODE"
    
    if [ "$CI_MODE" = true ]; then
        echo "❌ CI mode: Coverage below target threshold ($COVERAGE_TARGET%)"
        exit $COVERAGE_EXIT_CODE
    else
        echo "⚠️  Development mode: Continuing despite coverage issues..."
    fi
fi

COVERAGE_END_TIME=$(date +%s)
COVERAGE_DURATION=$((COVERAGE_END_TIME - COVERAGE_START_TIME))

# Parse coverage results
if [ -f "$COVERAGE_DIR/cobertura.xml" ]; then
    # Extract coverage percentage from XML (more reliable than stdout parsing)
    COVERAGE_PERCENT=$(grep -o 'line-rate="[0-9.]*"' "$COVERAGE_DIR/cobertura.xml" | head -1 | sed 's/line-rate="//;s/"//' | awk '{printf "%.2f", $1 * 100}')
else
    # Fallback to parsing stdout (less reliable)
    COVERAGE_PERCENT=$(tail -10 coverage_output.txt 2>/dev/null | grep -o '[0-9.]*% coverage' | sed 's/% coverage//' || echo "Unknown")
fi

# Generate coverage summary
echo ""
echo "📊 COVERAGE ANALYSIS RESULTS - $TIMESTAMP"
echo "========================================"
echo "🎯 Coverage Target: $COVERAGE_TARGET%"
echo "📈 Actual Coverage: $COVERAGE_PERCENT%"
echo "⏱️  Analysis Duration: ${COVERAGE_DURATION}s"
echo "📁 Report Location: $COVERAGE_DIR/"

if [ -f "$COVERAGE_DIR/tarpaulin-report.html" ]; then
    echo "🌐 HTML Report: file://$COVERAGE_DIR/tarpaulin-report.html"
fi

if [ -f "$COVERAGE_DIR/cobertura.xml" ]; then
    echo "🤖 XML Report: $COVERAGE_DIR/cobertura.xml"
fi

# Coverage quality assessment
if [ "$COVERAGE_PERCENT" != "Unknown" ]; then
    COVERAGE_NUM=$(echo "$COVERAGE_PERCENT" | awk '{print int($1)}')
    
    if [ "$COVERAGE_NUM" -ge "$COVERAGE_TARGET" ]; then
        echo "✅ Coverage target met! ($COVERAGE_PERCENT% >= $COVERAGE_TARGET%)"
        COVERAGE_STATUS="PASSED"
    else
        echo "⚠️  Coverage below target ($COVERAGE_PERCENT% < $COVERAGE_TARGET%)"
        COVERAGE_STATUS="BELOW_TARGET"
        
        if [ "$CI_MODE" = true ]; then
            echo "❌ CI mode: Failing build due to insufficient coverage"
            exit 1
        fi
    fi
else
    echo "⚠️  Could not determine coverage percentage"
    COVERAGE_STATUS="UNKNOWN"
fi

# Update quality metrics file with coverage data
QUALITY_METRICS_FILE="$(dirname "$PROJECT_DIR")/ai_assisted_development/QUALITY_METRICS.md"
if [ -f "$QUALITY_METRICS_FILE" ]; then
    echo ""
    echo "📝 Updating quality metrics..."
    
    # Create temporary coverage section
    TEMP_COVERAGE_SECTION=$(cat << EOF

## Test Coverage Analysis - $TIMESTAMP

- **Coverage Percentage**: $COVERAGE_PERCENT%
- **Coverage Target**: $COVERAGE_TARGET%
- **Status**: $COVERAGE_STATUS
- **Analysis Duration**: ${COVERAGE_DURATION}s
- **Report Generation**: $(date +'%Y-%m-%d %H:%M:%S')

### Coverage Breakdown
$(if [ -f "$COVERAGE_DIR/tarpaulin-report.html" ]; then
    echo "- HTML Report: \`coverage/tarpaulin-report.html\`"
fi)
$(if [ -f "$COVERAGE_DIR/cobertura.xml" ]; then
    echo "- XML Report: \`coverage/cobertura.xml\`"
fi)

### Quality Gates
- ✅ Pre-coverage clippy check: PASSED
- ✅ Code formatting check: PASSED  
- ✅ Compilation check: PASSED
- $(if [ "$COVERAGE_STATUS" = "PASSED" ]; then echo "✅"; else echo "⚠️ "; fi) Coverage target ($COVERAGE_TARGET%): $COVERAGE_STATUS

---
EOF
)
    
    # Note: In a real implementation, you'd update the quality metrics file here
    # For now, we'll just display what would be added
    echo "Would add coverage section to QUALITY_METRICS.md"
fi

# Open HTML report if requested
if [ "$OPEN_REPORT" = true ] && [ -f "$COVERAGE_DIR/tarpaulin-report.html" ]; then
    echo ""
    echo "🌐 Opening coverage report in browser..."
    
    if command -v xdg-open &> /dev/null; then
        xdg-open "$COVERAGE_DIR/tarpaulin-report.html"
    elif command -v open &> /dev/null; then
        open "$COVERAGE_DIR/tarpaulin-report.html"
    else
        echo "🔗 Please open manually: file://$COVERAGE_DIR/tarpaulin-report.html"
    fi
fi

# Generate actionable recommendations
echo ""
echo "🎯 NEXT STEPS & RECOMMENDATIONS"
echo "==============================="

if [ "$COVERAGE_STATUS" = "BELOW_TARGET" ]; then
    echo "📈 TO IMPROVE COVERAGE:"
    echo "  1. Add tests for uncovered code paths"
    echo "  2. Focus on modules with lowest coverage"
    echo "  3. Add edge case and error condition tests"
    echo "  4. Review test quality vs. quantity"
fi

echo "🔧 COVERAGE COMMANDS:"
echo "  • Run coverage:           ./scripts/coverage.sh"
echo "  • Run with report:        ./scripts/coverage.sh --open"
echo "  • CI/strict mode:         ./scripts/coverage.sh --ci"
echo "  • Custom target:          ./scripts/coverage.sh --target 85"

echo ""
echo "✨ Coverage analysis completed successfully!"

# Exit with appropriate code
if [ "$CI_MODE" = true ] && [ "$COVERAGE_STATUS" != "PASSED" ]; then
    exit 1
else
    exit 0
fi 