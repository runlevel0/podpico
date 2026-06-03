#!/bin/bash

# PodPico Quality Check Script
# Runs frontend and backend quality checks and exits non-zero if any required check fails.
# Use --allow-known-failures only for local baseline work while issue #2 is open.

set -u

ALLOW_KNOWN_FAILURES=false
for arg in "$@"; do
  case "$arg" in
    --allow-known-failures)
      ALLOW_KNOWN_FAILURES=true
      ;;
  esac
done

echo "🔍 PodPico Quality Check - $(date)"
echo "========================================"
if [[ "$ALLOW_KNOWN_FAILURES" == "true" ]]; then
  echo "Mode: --allow-known-failures (lint/test failures will not fail the script)"
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

FAILED_CHECKS=()
WARN_CHECKS=()

record_failure() {
  FAILED_CHECKS+=("$1")
}

record_warn() {
  WARN_CHECKS+=("$1")
}

should_fail() {
  if [[ "$ALLOW_KNOWN_FAILURES" == "true" ]]; then
    record_warn "$1"
    return 1
  fi
  record_failure "$1"
  return 0
}

# --- Backend ---

check_backend() {
  echo -e "\n📦 Backend Quality Checks"
  echo "-------------------------"

  cd src-tauri || exit 1

  echo "Running Rust clippy (-D warnings)..."
  if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✅ Clippy: PASSED${NC}"
  else
    echo -e "${RED}❌ Clippy: FAILED${NC}"
    record_failure "backend-clippy"
  fi

  echo "Checking Rust code formatting..."
  if cargo fmt --all -- --check; then
    echo -e "${GREEN}✅ Rust formatting: PASSED${NC}"
  else
    echo -e "${RED}❌ Rust formatting: FAILED${NC}"
    record_failure "backend-fmt"
  fi

  echo "Running backend tests..."
  BACKEND_TEST_OUTPUT=$(cargo test --all 2>&1) || BACKEND_TEST_EXIT=$?
  echo "$BACKEND_TEST_OUTPUT" | tail -20
  BACKEND_SUMMARY=$(echo "$BACKEND_TEST_OUTPUT" | rg "test result:" | tail -1 || true)
  if [[ -z "$BACKEND_SUMMARY" ]]; then
    BACKEND_SUMMARY="(could not parse test result line)"
  fi
  echo "Backend summary: $BACKEND_SUMMARY"
  if [[ "${BACKEND_TEST_EXIT:-0}" -eq 0 ]]; then
    echo -e "${GREEN}✅ Backend tests: PASSED${NC}"
  else
    echo -e "${RED}❌ Backend tests: FAILED${NC}"
    if should_fail "backend-tests"; then
      :
    else
      echo -e "${YELLOW}⚠️  Backend tests failed (allowed by --allow-known-failures)${NC}"
    fi
  fi

  cd ..
}

# --- Frontend ---

check_frontend() {
  echo -e "\n🌐 Frontend Quality Checks"
  echo "--------------------------"

  echo "Running TypeScript type checking..."
  if npm run type-check; then
    echo -e "${GREEN}✅ TypeScript: PASSED${NC}"
  else
    echo -e "${RED}❌ TypeScript: FAILED${NC}"
    record_failure "typescript"
  fi

  echo "Running ESLint (errors fail; warnings allowed)..."
  LINT_OUTPUT=$(npm run lint 2>&1) || LINT_EXIT=$?
  echo "$LINT_OUTPUT" | tail -15
  LINT_SUMMARY=$(echo "$LINT_OUTPUT" | rg "✖ .* problems" | tail -1 || true)
  if [[ -n "$LINT_SUMMARY" ]]; then
    echo "Lint summary: $LINT_SUMMARY"
  fi
  if [[ "${LINT_EXIT:-0}" -eq 0 ]]; then
    echo -e "${GREEN}✅ ESLint: PASSED${NC}"
  else
    echo -e "${RED}❌ ESLint: FAILED${NC}"
    if should_fail "eslint"; then
      :
    else
      echo -e "${YELLOW}⚠️  ESLint failed (allowed by --allow-known-failures)${NC}"
    fi
  fi

  echo "Checking Prettier formatting..."
  if npm run format:check; then
    echo -e "${GREEN}✅ Prettier: PASSED${NC}"
  else
    echo -e "${RED}❌ Prettier: FAILED${NC}"
    record_failure "prettier"
  fi

  echo "Running frontend tests..."
  FRONTEND_TEST_OUTPUT=$(npm run test:run 2>&1) || FRONTEND_TEST_EXIT=$?
  echo "$FRONTEND_TEST_OUTPUT" | tail -15
  FRONTEND_SUMMARY=$(echo "$FRONTEND_TEST_OUTPUT" | rg "Tests +.*failed.*passed" | tail -1 || true)
  if [[ -z "$FRONTEND_SUMMARY" ]]; then
    FRONTEND_SUMMARY=$(echo "$FRONTEND_TEST_OUTPUT" | rg "Tests .*passed" | tail -1 || true)
  fi
  if [[ -n "$FRONTEND_SUMMARY" ]]; then
    echo "Frontend summary: $FRONTEND_SUMMARY"
  fi
  if [[ "${FRONTEND_TEST_EXIT:-0}" -eq 0 ]]; then
    echo -e "${GREEN}✅ Frontend tests: PASSED${NC}"
  else
    echo -e "${RED}❌ Frontend tests: FAILED${NC}"
    if should_fail "frontend-tests"; then
      :
    else
      echo -e "${YELLOW}⚠️  Frontend tests failed (allowed by --allow-known-failures)${NC}"
    fi
  fi
}

check_build() {
  echo -e "\n🏗️  Build Verification"
  echo "----------------------"

  echo "Building TypeScript..."
  if npm run build; then
    echo -e "${GREEN}✅ TypeScript build: PASSED${NC}"
  else
    echo -e "${RED}❌ TypeScript build: FAILED${NC}"
    record_failure "build"
  fi

  cd src-tauri || exit 1
  echo "Checking backend compilation..."
  if cargo check --all-targets --all-features; then
    echo -e "${GREEN}✅ Backend compilation: PASSED${NC}"
  else
    echo -e "${RED}❌ Backend compilation: FAILED${NC}"
    record_failure "cargo-check"
  fi
  cd ..
}

check_documentation() {
  echo -e "\n📚 Documentation Verification"
  echo "-----------------------------"

  if ./scripts/doc-verify.sh; then
    echo -e "${GREEN}✅ Documentation verification: PASSED${NC}"
  else
    doc_exit_code=$?
    if [[ $doc_exit_code -eq 2 ]]; then
      echo -e "${YELLOW}⚠️  Documentation verification: WARNINGS${NC}"
      record_warn "doc-verify-warnings"
    else
      echo -e "${RED}❌ Documentation verification: FAILED${NC}"
      record_failure "doc-verify"
    fi
  fi
}

main() {
  check_backend
  check_frontend
  check_build
  check_documentation

  echo -e "\n📋 Quality Check Summary"
  echo "========================"
  echo "Canonical status: ai_assisted_development/PROGRESS.md#current-status"
  echo "See measured numbers from this run above (backend/frontend test result lines, lint summary)."

  if [[ ${#WARN_CHECKS[@]} -gt 0 ]]; then
    echo -e "${YELLOW}Allowed failures:${NC} ${WARN_CHECKS[*]}"
  fi

  if [[ ${#FAILED_CHECKS[@]} -gt 0 ]]; then
    echo -e "${RED}❌ Failed checks:${NC} ${FAILED_CHECKS[*]}"
    echo "Fix failures or track in issue #2. For local-only baseline: --allow-known-failures"
    exit 1
  fi

  echo -e "${GREEN}✅ All required quality checks passed.${NC}"
  exit 0
}

main
