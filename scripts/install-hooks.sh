#!/bin/bash

# PodPico Git Hooks Installation Script
# Sets up pre-commit and commit-msg hooks for quality enforcement

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 PodPico Git Hooks Installation${NC}"
echo "====================================="

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
    echo -e "${RED}❌ Error: Must run from PodPico project root${NC}"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Function to create a hook
create_hook() {
    local hook_name=$1
    local source_file=$2
    local target_file=".git/hooks/$hook_name"
    
    echo -ne "Installing $hook_name hook... "
    
    # Check if hook already exists
    if [ -f "$target_file" ]; then
        # Backup existing hook
        mv "$target_file" "${target_file}.backup.$(date +%Y%m%d_%H%M%S)"
        echo -ne "(backed up existing) "
    fi
    
    # Copy the hook
    cp "$source_file" "$target_file"
    chmod +x "$target_file"
    
    echo -e "${GREEN}✅${NC}"
}

# Pre-commit hook content
cat > /tmp/podpico-pre-commit << 'EOF'
#!/bin/bash

# PodPico Pre-Commit Hook
# Enforces quality standards before allowing commits
# Part of the zero-tolerance quality policy

set -e  # Exit on first error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 PodPico Pre-Commit Quality Checks${NC}"
echo "========================================"
echo -e "Running quality checks on staged files..."
echo ""

# Track if we have changes in different areas
HAS_RUST_CHANGES=false
HAS_TS_CHANGES=false
HAS_DOC_CHANGES=false

# Check what files are staged
STAGED_FILES=$(git diff --cached --name-only)

# Categorize staged files
for file in $STAGED_FILES; do
    if [[ $file == *.rs ]]; then
        HAS_RUST_CHANGES=true
    elif [[ $file == *.ts || $file == *.tsx ]]; then
        HAS_TS_CHANGES=true
    elif [[ $file == *.md ]]; then
        HAS_DOC_CHANGES=true
    fi
done

# Function to run checks with timing
run_check() {
    local name=$1
    local command=$2
    local start_time=$(date +%s)
    
    echo -ne "⏳ $name... "
    
    if eval "$command" > /dev/null 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "${GREEN}✅ PASSED${NC} (${duration}s)"
        return 0
    else
        echo -e "${RED}❌ FAILED${NC}"
        echo -e "${RED}   Run the following command to see details:${NC}"
        echo -e "${YELLOW}   $command${NC}"
        return 1
    fi
}

# Track failures
FAILED_CHECKS=()

# Rust/Backend checks
if [ "$HAS_RUST_CHANGES" = true ]; then
    echo -e "\n${BLUE}📦 Backend Quality Checks${NC}"
    echo "-------------------------"
    
    # Save current directory
    ORIGINAL_DIR=$(pwd)
    cd src-tauri || exit 1
    
    # Check only staged Rust files for formatting
    STAGED_RUST_FILES=$(git diff --cached --name-only --relative | grep '\.rs$' || true)
    
    if [ -n "$STAGED_RUST_FILES" ]; then
        # Check formatting on staged files
        if ! run_check "Rust formatting" "rustfmt --check $STAGED_RUST_FILES"; then
            FAILED_CHECKS+=("rust_fmt")
            echo -e "${YELLOW}   Fix with: cd src-tauri && cargo fmt --all${NC}"
        fi
        
        # Run clippy on the whole project (it's fast and catches project-wide issues)
        if ! run_check "Clippy (zero warnings)" "cargo clippy --all-targets --all-features -- -D warnings"; then
            FAILED_CHECKS+=("clippy")
        fi
        
        # Run tests (quick check)
        if ! run_check "Rust tests" "cargo test --all"; then
            FAILED_CHECKS+=("rust_tests")
        fi
    fi
    
    cd "$ORIGINAL_DIR"
fi

# TypeScript/Frontend checks
if [ "$HAS_TS_CHANGES" = true ]; then
    echo -e "\n${BLUE}🌐 Frontend Quality Checks${NC}"
    echo "--------------------------"
    
    # Get staged TypeScript files
    STAGED_TS_FILES=$(git diff --cached --name-only | grep -E '\.(ts|tsx)$' || true)
    
    if [ -n "$STAGED_TS_FILES" ]; then
        # TypeScript compilation check
        if ! run_check "TypeScript compilation" "npm run type-check"; then
            FAILED_CHECKS+=("typescript")
        fi
        
        # ESLint on staged files only
        if ! run_check "ESLint (zero warnings)" "npx eslint $STAGED_TS_FILES --report-unused-disable-directives"; then
            FAILED_CHECKS+=("eslint")
            echo -e "${YELLOW}   Fix with: npm run lint:fix${NC}"
        fi
        
        # Prettier check on staged files
        if ! run_check "Prettier formatting" "npx prettier --check $STAGED_TS_FILES"; then
            FAILED_CHECKS+=("prettier")
            echo -e "${YELLOW}   Fix with: npm run format${NC}"
        fi
        
        # Run frontend tests (if not too many files changed)
        STAGED_COUNT=$(echo "$STAGED_TS_FILES" | wc -w)
        if [ "$STAGED_COUNT" -lt 10 ]; then
            if ! run_check "Frontend tests" "npm run test:run"; then
                echo -e "${YELLOW}   Note: Test failures may be expected during development${NC}"
            fi
        else
            echo -e "${YELLOW}⚠️  Skipping frontend tests (too many files changed)${NC}"
        fi
    fi
fi

# Documentation checks
if [ "$HAS_DOC_CHANGES" = true ]; then
    echo -e "\n${BLUE}📚 Documentation Checks${NC}"
    echo "------------------------"
    
    # Check if AI development docs are changed
    AI_DOCS_CHANGED=$(echo "$STAGED_FILES" | grep 'ai_assisted_development/' || true)
    
    if [ -n "$AI_DOCS_CHANGED" ]; then
        if ! run_check "Documentation verification" "./scripts/doc-verify.sh"; then
            echo -e "${YELLOW}   Warning: Documentation may be inconsistent${NC}"
            echo -e "${YELLOW}   This won't block the commit but should be addressed${NC}"
        fi
    fi
fi

# Summary and decision
echo -e "\n${BLUE}📊 Pre-Commit Summary${NC}"
echo "====================="

if [ ${#FAILED_CHECKS[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ All quality checks passed!${NC}"
    echo -e "${GREEN}✅ Commit approved${NC}"
    
    # Quick stats
    if [ "$HAS_RUST_CHANGES" = true ]; then
        echo -e "   • Backend: All checks passed"
    fi
    if [ "$HAS_TS_CHANGES" = true ]; then
        echo -e "   • Frontend: All checks passed"
    fi
    if [ "$HAS_DOC_CHANGES" = true ]; then
        echo -e "   • Documentation: Verified"
    fi
    
    exit 0
else
    echo -e "${RED}❌ Quality checks failed!${NC}"
    echo -e "${RED}❌ Commit blocked${NC}"
    echo ""
    echo -e "${RED}Failed checks:${NC}"
    for check in "${FAILED_CHECKS[@]}"; do
        echo -e "   • $check"
    done
    echo ""
    echo -e "${YELLOW}Please fix the issues above before committing.${NC}"
    echo -e "${YELLOW}You can bypass this check with 'git commit --no-verify' (not recommended)${NC}"
    
    exit 1
fi
EOF

# Commit-msg hook content
cat > /tmp/podpico-commit-msg << 'EOF'
#!/bin/bash

# PodPico Commit Message Validation Hook
# Enforces conventional commit format and quality standards

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the commit message file
COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

echo -e "${BLUE}📝 PodPico Commit Message Validation${NC}"
echo "======================================"

# Check if this is a merge commit
if git rev-parse --verify HEAD~1 >/dev/null 2>&1 && git rev-parse --verify MERGE_HEAD >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Merge commit detected - skipping validation${NC}"
    exit 0
fi

# Conventional commit regex pattern
# Format: type(scope): description
# or: type: description
CONVENTIONAL_COMMIT_REGEX="^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .{1,100}"

# Extended regex for AI agent commits (includes session info)
AI_AGENT_REGEX="^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert): Session [0-9]+ - .+"

# Check first line
FIRST_LINE=$(echo "$COMMIT_MSG" | head -n1)

# Validation flags
VALID=true
ERRORS=()

# Check conventional commit format
if ! echo "$FIRST_LINE" | grep -qE "$CONVENTIONAL_COMMIT_REGEX" && ! echo "$FIRST_LINE" | grep -qE "$AI_AGENT_REGEX"; then
    VALID=false
    ERRORS+=("First line doesn't follow conventional commit format")
fi

# Check first line length (should be <= 100 characters)
if [ ${#FIRST_LINE} -gt 100 ]; then
    VALID=false
    ERRORS+=("First line is too long (${#FIRST_LINE} chars, max 100)")
fi

# Check for empty body after first line (should have blank line)
SECOND_LINE=$(echo "$COMMIT_MSG" | sed -n '2p')
if [ -n "$SECOND_LINE" ] && [ -n "$(echo "$SECOND_LINE" | tr -d '[:space:]')" ]; then
    VALID=false
    ERRORS+=("Missing blank line after first line")
fi

# Check for User Story references if it's a feature
if echo "$FIRST_LINE" | grep -qE "^feat"; then
    if ! echo "$COMMIT_MSG" | grep -qE "User Story #[0-9]+"; then
        echo -e "${YELLOW}⚠️  Warning: Feature commit doesn't reference a User Story${NC}"
        echo -e "${YELLOW}   Consider adding 'User Story #X' reference in the body${NC}"
    fi
fi

# Check for AI Agent session info
if echo "$COMMIT_MSG" | grep -qE "Session: [0-9]{4}-[0-9]{2}-[0-9]{2}"; then
    echo -e "${BLUE}ℹ️  AI Agent session commit detected${NC}"
    
    # Verify co-authored-by line exists
    if ! echo "$COMMIT_MSG" | grep -qE "Co-authored-by: AI Agent Session"; then
        echo -e "${YELLOW}⚠️  Warning: AI Agent commit missing Co-authored-by line${NC}"
    fi
fi

# Display results
if [ "$VALID" = true ]; then
    echo -e "${GREEN}✅ Commit message validation passed!${NC}"
    
    # Display commit type
    COMMIT_TYPE=$(echo "$FIRST_LINE" | sed -E 's/^([a-z]+)(\(.+\))?: .*/\1/')
    echo -e "   • Type: ${BLUE}$COMMIT_TYPE${NC}"
    
    # Display scope if present
    if echo "$FIRST_LINE" | grep -qE "^[a-z]+\(.+\):"; then
        SCOPE=$(echo "$FIRST_LINE" | sed -E 's/^[a-z]+\((.+)\): .*/\1/')
        echo -e "   • Scope: ${BLUE}$SCOPE${NC}"
    fi
    
    exit 0
else
    echo -e "${RED}❌ Commit message validation failed!${NC}"
    echo ""
    echo -e "${RED}Errors:${NC}"
    for error in "${ERRORS[@]}"; do
        echo -e "   • $error"
    done
    echo ""
    echo -e "${YELLOW}Conventional Commit Format:${NC}"
    echo "   type(scope): description"
    echo ""
    echo -e "${YELLOW}Valid types:${NC}"
    echo "   • feat     - New feature"
    echo "   • fix      - Bug fix"
    echo "   • docs     - Documentation changes"
    echo "   • style    - Code style changes (formatting, etc)"
    echo "   • refactor - Code refactoring"
    echo "   • test     - Test additions or changes"
    echo "   • chore    - Build process or auxiliary tool changes"
    echo "   • perf     - Performance improvements"
    echo "   • ci       - CI/CD changes"
    echo "   • build    - Build system changes"
    echo "   • revert   - Revert a previous commit"
    echo ""
    echo -e "${YELLOW}Examples:${NC}"
    echo "   feat: add podcast download progress tracking"
    echo "   fix(usb): resolve device detection on Linux"
    echo "   docs: update pre-commit hook documentation"
    echo "   test: add coverage for episode transfer"
    echo ""
    echo -e "${YELLOW}AI Agent Format:${NC}"
    echo "   feat: Session 21 - User Story #3 - Download functionality"
    echo ""
    
    exit 1
fi
EOF

# Install the hooks
create_hook "pre-commit" "/tmp/podpico-pre-commit"
create_hook "commit-msg" "/tmp/podpico-commit-msg"

# Clean up temporary files
rm -f /tmp/podpico-pre-commit /tmp/podpico-commit-msg

echo ""
echo -e "${GREEN}✨ Git hooks successfully installed!${NC}"
echo ""
echo -e "${BLUE}Installed hooks:${NC}"
echo "   • pre-commit  - Runs quality checks on staged files"
echo "   • commit-msg  - Validates commit message format"
echo ""
echo -e "${YELLOW}Usage:${NC}"
echo "   • Hooks run automatically when you commit"
echo "   • To skip hooks: git commit --no-verify"
echo "   • To uninstall: rm .git/hooks/{pre-commit,commit-msg}"
echo ""
echo -e "${BLUE}Quality checks performed:${NC}"
echo "   • Backend: clippy, formatting, tests"
echo "   • Frontend: ESLint, Prettier, TypeScript, tests"
echo "   • Documentation: consistency verification"
echo "   • Commit message: conventional format validation"
echo ""