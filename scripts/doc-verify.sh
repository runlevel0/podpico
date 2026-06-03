#!/bin/bash

# PodPico Documentation Verification Script
# Ensures all AI agent documentation is up-to-date and consistent

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Documentation files to verify
DOCS_DIR="ai_assisted_development"
REQUIRED_DOCS=(
    "AI_AGENT_INSTRUCTIONS.md"
    "QUALITY_METRICS.md"
    "ISSUES.md"
    "SESSION_NOTES.md"
    "PROGRESS.md"
)

# Get current date and recent dates for verification
CURRENT_DATE=$(date +"%Y-%m-%d")
CURRENT_DATETIME=$(date +"%Y-%m-%d %H:%M:%S")
WEEK_AGO=$(date -d '7 days ago' +"%Y-%m-%d")

echo -e "${BLUE}🔍 PodPico Documentation Verification${NC}"
echo "=================================================="
echo "Current Date: $CURRENT_DATE"
echo "Verification Time: $CURRENT_DATETIME"
echo ""

# Check if all required documentation files exist
echo -e "${BLUE}📁 Checking Documentation File Existence...${NC}"
missing_files=0
for doc in "${REQUIRED_DOCS[@]}"; do
    if [[ -f "$DOCS_DIR/$doc" ]]; then
        echo -e "  ✅ $doc exists"
    else
        echo -e "  ❌ $doc is missing"
        missing_files=$((missing_files + 1))
    fi
done

if [[ $missing_files -gt 0 ]]; then
    echo -e "\n${RED}❌ Documentation Verification Failed: $missing_files missing files${NC}"
    exit 1
fi

# Function to check if file was updated recently (within last 24 hours)
check_recent_update() {
    local file="$1"
    local file_date=$(stat -c %Y "$file")
    local current_timestamp=$(date +%s)
    local day_ago=$((current_timestamp - 86400))  # 24 hours ago
    
    if [[ $file_date -gt $day_ago ]]; then
        return 0  # Recently updated
    else
        return 1  # Not recently updated
    fi
}

# Function to extract last updated date from file content
get_last_updated_from_content() {
    local file="$1"
    local date_pattern="[0-9]{4}-[0-9]{2}-[0-9]{2}"
    local datetime_pattern="[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}"
    
    # Look for "Last Updated:", "Updated:", or similar patterns
    grep -E "(Last Updated|Updated|Session [0-9]+ -)" "$file" | head -1 | grep -oE "$datetime_pattern|$date_pattern" | head -1 || echo ""
}

echo -e "\n${BLUE}📅 Checking Documentation Freshness...${NC}"
stale_docs=0
for doc in "${REQUIRED_DOCS[@]}"; do
    doc_path="$DOCS_DIR/$doc"
    content_date=$(get_last_updated_from_content "$doc_path")
    
    if [[ -n "$content_date" ]]; then
        if [[ "$content_date" == "$CURRENT_DATE"* ]] || [[ "$content_date" > "$WEEK_AGO" ]]; then
            echo -e "  ✅ $doc is up to date (Last: $content_date)"
        else
            echo -e "  ⚠️  $doc may be stale (Last: $content_date)"
            stale_docs=$((stale_docs + 1))
        fi
    else
        echo -e "  ❓ $doc has no detectable update date"
        stale_docs=$((stale_docs + 1))
    fi
done

# Verify specific content requirements
echo -e "\n${BLUE}🔍 Checking Content Consistency...${NC}"

# Canonical status section (single source of truth)
if grep -q "## Current Status" "$DOCS_DIR/PROGRESS.md"; then
    echo -e "  ✅ PROGRESS.md has canonical ## Current Status section"
else
    echo -e "  ❌ PROGRESS.md missing ## Current Status (canonical status block)"
    stale_docs=$((stale_docs + 1))
fi

if grep -q "PROGRESS.md#current-status" README.md; then
    echo -e "  ✅ README.md references PROGRESS.md#current-status"
else
    echo -e "  ❌ README.md should reference ai_assisted_development/PROGRESS.md#current-status"
    stale_docs=$((stale_docs + 1))
fi

if grep -q "PROGRESS.md#current-status" "$DOCS_DIR/QUALITY_METRICS.md"; then
    echo -e "  ✅ QUALITY_METRICS.md references canonical PROGRESS status"
else
    echo -e "  ❌ QUALITY_METRICS.md should reference PROGRESS.md#current-status"
    stale_docs=$((stale_docs + 1))
fi

if grep -q "Measured gate results" "$DOCS_DIR/QUALITY_METRICS.md"; then
    echo -e "  ✅ QUALITY_METRICS.md contains measured gate results"
else
    echo -e "  ❌ QUALITY_METRICS.md missing measured gate results table"
    stale_docs=$((stale_docs + 1))
fi

# Check ISSUES.md for resolved vs open issues
resolved_count=$(grep -c "✅ RESOLVED" "$DOCS_DIR/ISSUES.md" || echo "0")
open_count=$(grep -c "❌ OPEN" "$DOCS_DIR/ISSUES.md" || echo "0")
echo -e "  ✅ ISSUES.md: $resolved_count resolved, $open_count open issues"

# Check SESSION_NOTES.md for recent session entries
recent_session=$(grep -E "Session [0-9]+ -.*$CURRENT_DATE" "$DOCS_DIR/SESSION_NOTES.md" || echo "")
if [[ -n "$recent_session" ]]; then
    echo -e "  ✅ SESSION_NOTES.md has recent session entry"
else
    echo -e "  ⚠️  SESSION_NOTES.md may be missing today's session"
    stale_docs=$((stale_docs + 1))
fi

# Advanced consistency checks
echo -e "\n${BLUE}🔧 Advanced Consistency Checks...${NC}"

# Flag inflated success claims in doc summaries (not negations, targets, or historical disclaimers)
INFLATED_PATTERNS='107/107 ✅|EXCELLENT – PRODUCTION READY|Overall Quality.*92%|Backend Tests.*107/107'
inflated_found=0
if head -n 80 README.md | grep -qE "$INFLATED_PATTERNS"; then
    echo -e "  ⚠️  README.md may contain unsupported inflated claims"
    inflated_found=1
fi
if head -n 55 "$DOCS_DIR/QUALITY_METRICS.md" | grep -qE "$INFLATED_PATTERNS"; then
    echo -e "  ⚠️  QUALITY_METRICS.md summary may contain unsupported inflated claims"
    inflated_found=1
fi
if [[ $inflated_found -eq 1 ]]; then
    stale_docs=$((stale_docs + 1))
else
    echo -e "  ✅ README and QUALITY_METRICS summaries avoid inflated success claim patterns"
fi

# Check if all critical issues marked as resolved have resolution details
echo -e "\n${BLUE}🎯 Critical Issue Resolution Verification...${NC}"
critical_resolved=$(grep -A 2 "CRITICAL.*RESOLVED" "$DOCS_DIR/ISSUES.md" || echo "")
if [[ -n "$critical_resolved" ]]; then
    resolution_count=$(echo "$critical_resolved" | grep -c "Resolution:" || echo "0")
    critical_count=$(echo "$critical_resolved" | grep -c "CRITICAL.*RESOLVED" || echo "0")
    if [[ "$resolution_count" -eq "$critical_count" ]]; then
        echo -e "  ✅ All resolved critical issues have resolution details"
    else
        echo -e "  ❌ Some resolved critical issues missing resolution details"
        stale_docs=$((stale_docs + 1))
    fi
fi

# Final assessment
echo -e "\n${BLUE}📊 Documentation Verification Summary${NC}"
echo "=================================================="

if [[ $stale_docs -eq 0 ]]; then
    echo -e "${GREEN}✅ ALL DOCUMENTATION VERIFICATION PASSED${NC}"
    echo -e "   📚 All required documentation files present"
    echo -e "   📅 All documentation appears up to date"
    echo -e "   🔄 Content consistency checks passed"
    echo -e "   ✨ Documentation is ready for development"
    exit 0
else
    echo -e "${YELLOW}⚠️  DOCUMENTATION VERIFICATION WARNINGS: $stale_docs issues found${NC}"
    echo -e "   📝 Consider updating stale documentation"
    echo -e "   🔄 Some consistency checks failed"
    echo ""
    echo -e "${BLUE}📋 Required Documentation Updates:${NC}"
    echo "   1. Update timestamps in stale documents"  
    echo "   2. Ensure session notes reflect recent work"
    echo "   3. Verify progress percentages are current"
    echo "   4. Check that issue statuses are accurate"
    echo "   5. Confirm quality metrics match current state"
    echo ""
    echo -e "${YELLOW}⏱️  Run this check again after updating documentation${NC}"
    exit 2  # Warning exit code, not failure
fi 