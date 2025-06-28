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

# Check PROGRESS.md for current phase and completion percentage
if grep -q "Phase 1" "$DOCS_DIR/PROGRESS.md" && grep -q "%" "$DOCS_DIR/PROGRESS.md"; then
    phase_info=$(grep -E "Phase [0-9]|[0-9]+%" "$DOCS_DIR/PROGRESS.md" | head -2)
    echo -e "  ✅ PROGRESS.md contains phase and completion info"
    echo "     $phase_info" | tr '\n' ' '
    echo ""
else
    echo -e "  ❌ PROGRESS.md missing phase/completion information"
    stale_docs=$((stale_docs + 1))
fi

# Check QUALITY_METRICS.md for recent test results
if grep -q "tests passing" "$DOCS_DIR/QUALITY_METRICS.md" && grep -q "Status.*:" "$DOCS_DIR/QUALITY_METRICS.md"; then
    echo -e "  ✅ QUALITY_METRICS.md contains test status"
else
    echo -e "  ❌ QUALITY_METRICS.md missing test status information"
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

# Check if backend test count is consistent across docs
backend_tests_quality=$(grep -oE "[0-9]+ tests passing" "$DOCS_DIR/QUALITY_METRICS.md" | head -1 | grep -oE "[0-9]+" || echo "")
backend_tests_progress=$(grep -oE "[0-9]+ tests passing" "$DOCS_DIR/PROGRESS.md" | head -1 | grep -oE "[0-9]+" || echo "")

if [[ -n "$backend_tests_quality" ]] && [[ -n "$backend_tests_progress" ]]; then
    if [[ "$backend_tests_quality" == "$backend_tests_progress" ]]; then
        echo -e "  ✅ Backend test counts consistent across documents ($backend_tests_quality tests)"
    else
        echo -e "  ❌ Backend test count mismatch: QUALITY_METRICS.md($backend_tests_quality) vs PROGRESS.md($backend_tests_progress)"
        stale_docs=$((stale_docs + 1))
    fi
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