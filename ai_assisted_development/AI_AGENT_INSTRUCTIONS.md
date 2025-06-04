# AI Agent Development Instructions

This document provides comprehensive instructions for AI agents taking over PodPico development sessions.

## 🚀 Session Startup Checklist

### 1. Environment Verification
Before starting any development work:

```bash
# Verify you're in the correct directory
pwd # Should be /home/patrick/Workspaces/podpico

# Check system tools
node --version    # Should be 22.16.0+
npm --version     # Should be 10.x+
rustc --version   # Should be 1.87.0+
cargo --version   # Should be 1.87.0+

# MANDATORY: Run linting and fix all issues
cargo clippy --all-targets --all-features -- -D warnings
npm run lint || echo "Frontend linting not configured yet"

# MANDATORY: Run all tests and ensure they pass
cargo test --all
npm test || echo "Frontend tests not configured yet"

# Test basic compilation with zero warnings
cd src-tauri && cargo check --all-targets --all-features
cd .. && npm run tauri dev --help
```

### 2. Context Loading
Read these files in order to understand current state and product requirements:
1. **`ai_assisted_development/ProductOverview.md`** - Product vision, user stories, acceptance criteria, and UI specifications
2. **`ai_assisted_development/PROGRESS.md`** - Current phase, completed tasks, next priorities
3. **`ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`** - Complete development guidelines (this file)
4. **`ai_assisted_development/SESSION_NOTES.md`** - Detailed history of previous sessions
5. **`ai_assisted_development/ISSUES.md`** - Known blockers and technical debt
6. **`ai_assisted_development/QUALITY_METRICS.md`** - Current quality status and targets
7. **`ai_assisted_development/ImplementationPlan.md`** - Overall project architecture and timeline

### 3. Current State Assessment
```bash
# MANDATORY: Check compilation with zero tolerance for warnings
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ STOP: Fix all clippy warnings before proceeding"
    exit 1
fi

# MANDATORY: Run all existing tests
cargo test --all
if [ $? -ne 0 ]; then
    echo "❌ STOP: All tests must pass before proceeding"
    exit 1
fi

# Check what compiles
cargo check --all

# Look for any new/modified files
git status

# Review recent commits
git log --oneline -10

# Check if the app runs
npm run tauri dev
```

## 🧪 MANDATORY TESTING & QUALITY PROTOCOL

### Pre-Development Quality Gates
**EVERY SESSION MUST START WITH THESE PASSING:**

```bash
# 1. Zero compilation warnings allowed
cargo clippy --all-targets --all-features -- -D warnings

# 2. All tests must pass
cargo test --all

# 3. Code formatting must be consistent
cargo fmt --all -- --check

# 4. Frontend linting (when configured)
npm run lint

# 5. Security audit
cargo audit || echo "Consider running: cargo install cargo-audit"
```

### Test-First Development Protocol
**MANDATORY: Write tests BEFORE implementing features**

```rust
// Example: User Story #3 test BEFORE implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_story_3_download_episode_acceptance_criteria() {
        // Acceptance Criteria: Episode download completes within reasonable time
        // Acceptance Criteria: Progress indicator shows during download
        // Acceptance Criteria: Downloaded episode marked as available for transfer
        
        // Arrange
        let episode_id = create_test_episode().await;
        let start_time = std::time::Instant::now();
        
        // Act
        let result = download_episode(episode_id).await;
        let elapsed = start_time.elapsed();
        
        // Assert
        assert!(result.is_ok(), "Episode download should succeed");
        assert!(elapsed < Duration::from_secs(30), "Download should complete within 30 seconds");
        
        // Verify episode marked as downloaded
        let episode = get_episode(episode_id).await.unwrap();
        assert!(episode.downloaded, "Episode should be marked as downloaded");
    }

    #[tokio::test]
    async fn test_user_story_3_download_progress_tracking() {
        // Acceptance Criteria: User can see download progress percentage
        // Implementation should provide progress callbacks
        
        let episode_id = create_test_episode().await;
        let mut progress_updates = Vec::new();
        
        let result = download_episode_with_progress(episode_id, |progress| {
            progress_updates.push(progress);
        }).await;
        
        assert!(result.is_ok());
        assert!(!progress_updates.is_empty(), "Should receive progress updates");
        assert!(progress_updates.last().unwrap() >= &100.0, "Should reach 100% progress");
    }
}
```

### Continuous Quality Checking
**RUN AFTER EVERY SIGNIFICANT CHANGE:**

```bash
# After each function/module implemented:
cargo clippy --all-targets --all-features -- -D warnings  # Fix ALL warnings
cargo test --all                                          # ALL tests must pass
cargo fmt --all                                          # Auto-format code

# After each user story feature completed:
npm run tauri dev  # Verify app still runs
# Manual testing against acceptance criteria
```

## 📋 Session Planning Template

Copy this template to start each session:

```markdown
# Session [N] - [DATE] - Phase [X], Week [Y]

## Pre-Session Quality Checklist ⚠️ MANDATORY
- [ ] ✅ cargo clippy passes with zero warnings
- [ ] ✅ cargo test passes (100% existing tests)
- [ ] ✅ cargo fmt applied and code formatted
- [ ] ✅ Read ai_assisted_development/ProductOverview.md for product context and user stories
- [ ] ✅ Read all progress tracking files
- [ ] ✅ Verified development environment works
- [ ] ✅ Identified current state from git status
- [ ] ✅ Application compiles and runs successfully

## Test-Driven Development Plan ⚠️ MANDATORY
Before implementing ANY feature:
- [ ] Write failing test for user story acceptance criteria
- [ ] Implement minimum code to make test pass
- [ ] Refactor while keeping tests green
- [ ] Add additional tests for edge cases

## Session Objectives
Primary objectives (must complete):
- [ ] [Specific, measurable objective 1 - reference user story #X]
  - [ ] Tests written FIRST for acceptance criteria
  - [ ] Implementation passes all tests
  - [ ] Manual validation against acceptance criteria
- [ ] [Specific, measurable objective 2 - reference user story #Y]
  - [ ] Tests written FIRST for acceptance criteria  
  - [ ] Implementation passes all tests
  - [ ] Manual validation against acceptance criteria

Secondary objectives (if time permits):
- [ ] [Nice-to-have objective 1]
- [ ] [Nice-to-have objective 2]

## User Stories Addressed
- **User Story #X**: [Brief description from ai_assisted_development/ProductOverview.md]
  - Acceptance Criteria: [List key criteria from ai_assisted_development/ProductOverview.md]
  - Test Coverage Required: [List specific test cases needed]
- **User Story #Y**: [Brief description from ai_assisted_development/ProductOverview.md]
  - Acceptance Criteria: [List key criteria from ai_assisted_development/ProductOverview.md]
  - Test Coverage Required: [List specific test cases needed]

## Quality Gates ⚠️ MANDATORY FOR COMPLETION
- [ ] ALL clippy warnings resolved
- [ ] ALL tests pass (existing + new)
- [ ] Test coverage ≥80% for new code
- [ ] Manual acceptance criteria validation completed
- [ ] No security vulnerabilities introduced
- [ ] Performance requirements met (timing against acceptance criteria)
- [ ] Error handling comprehensive with user-friendly messages
- [ ] Logging added with user story context

## Implementation Plan (Test-First)
1. **[Task 1]** (Est: X minutes) - Addresses User Story #X
   - [ ] Write failing tests for acceptance criteria
   - [ ] Implement minimum viable solution
   - [ ] Ensure all tests pass
   - [ ] Manual validation against acceptance criteria
   - [ ] Run full quality check (clippy, tests, formatting)

2. **[Task 2]** (Est: X minutes) - Addresses User Story #Y
   - [ ] Write failing tests for acceptance criteria
   - [ ] Implement minimum viable solution
   - [ ] Ensure all tests pass
   - [ ] Manual validation against acceptance criteria
   - [ ] Run full quality check (clippy, tests, formatting)

## Continuous Quality Protocol
After EACH significant change:
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all
- [ ] Manual functionality test
- [ ] Commit only if ALL quality gates pass
```

## 🎯 Development Guidelines

### Test-Driven Development (MANDATORY)

#### Always Write Tests First
For every user story implementation:
1. **Write failing test** that validates acceptance criteria
2. **Implement minimum code** to make test pass
3. **Refactor** while keeping tests green
4. **Add comprehensive tests** for edge cases and error conditions

#### Example: User Story #3 Test-First Implementation
```rust
// STEP 1: Write failing test FIRST
#[tokio::test]
async fn test_user_story_3_download_episode() {
    // This test will fail initially - that's expected
    let episode_id = 1;
    let result = download_episode(episode_id).await;
    assert!(result.is_ok());
}

// STEP 2: Implement minimum code to pass test
#[tauri::command]
pub async fn download_episode(episode_id: i64) -> Result<(), String> {
    log::info!("Downloading episode {} (User Story #3)", episode_id);
    
    // Minimum implementation - will expand based on acceptance criteria
    Ok(())
}

// STEP 3: Enhance test to match acceptance criteria
#[tokio::test]
async fn test_user_story_3_acceptance_criteria() {
    // Acceptance Criteria: Progress tracking during download
    // Acceptance Criteria: Episode marked as downloaded when complete
    // Acceptance Criteria: Error handling for network failures
    
    let episode_id = create_test_episode().await;
    
    // Test progress tracking
    let mut progress_received = false;
    let result = download_episode_with_progress(episode_id, |_| {
        progress_received = true;
    }).await;
    
    assert!(result.is_ok());
    assert!(progress_received, "Should receive progress updates");
    
    // Test episode status update
    let episode = get_episode(episode_id).await.unwrap();
    assert!(episode.downloaded, "Episode should be marked as downloaded");
}
```

### Code Quality Standards (ZERO TOLERANCE)

#### Rust Backend Standards
```rust
// ✅ MANDATORY: All functions must have comprehensive error handling
async fn add_podcast(rss_url: String) -> Result<Podcast, PodPicoError> {
    log::info!("Adding podcast: {} (User Story #1)", rss_url);
    
    // MANDATORY: Input validation
    if rss_url.trim().is_empty() {
        return Err(PodPicoError::InvalidRssUrl("Empty URL".to_string()));
    }
    
    // MANDATORY: Timeout handling (User Story #1: validate within 5 seconds)
    let timeout = Duration::from_secs(5);
    let validation_result = timeout(timeout, validate_rss_feed(&rss_url)).await;
    
    match validation_result {
        Ok(Ok(podcast_info)) => {
            let podcast = create_podcast_from_info(podcast_info).await?;
            log::info!("User Story #1 completed successfully: {}", podcast.name);
            Ok(podcast)
        },
        Ok(Err(e)) => {
            log::warn!("User Story #1 validation failed: {}", e);
            Err(PodPicoError::InvalidRssUrl(format!("Invalid RSS feed: {}", e)))
        },
        Err(_) => {
            log::error!("User Story #1 timeout: Feed validation exceeded 5 seconds");
            Err(PodPicoError::NetworkTimeout("Feed validation timed out after 5 seconds".to_string()))
        }
    }
}

// ❌ FORBIDDEN: Any code without proper error handling
fn bad_example(rss_url: String) -> Podcast {
    rss_url.parse().unwrap() // ❌ Can panic - absolutely forbidden
}
```

#### Linting Standards (ZERO WARNINGS ALLOWED)
```bash
# MANDATORY: Must pass before ANY commit
cargo clippy --all-targets --all-features -- -D warnings

# Common issues to fix immediately:
# - Unused imports: Remove them
# - Unused variables: Use them or prefix with _underscore
# - Inefficient code: Follow clippy suggestions
# - Missing documentation: Add comprehensive comments
# - Unreachable code: Remove or fix logic
```

#### Testing Standards (MANDATORY COVERAGE)
```rust
// MANDATORY: Every public function needs tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test success case (happy path)
    #[tokio::test]
    async fn test_function_success() {
        // Test implementation
    }

    // Test error cases (all error branches)
    #[tokio::test]
    async fn test_function_invalid_input() {
        // Test error handling
    }

    // Test acceptance criteria (user story validation)
    #[tokio::test]
    async fn test_user_story_acceptance_criteria() {
        // Test against specific acceptance criteria
    }

    // Test performance requirements (when applicable)
    #[tokio::test]
    async fn test_performance_requirements() {
        // Ensure timing requirements met
    }
}
```

## 🔄 Session Workflow (UPDATED)

### 1. Start with Quality Verification
```bash
# ❌ DO NOT PROCEED IF ANY OF THESE FAIL
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo fmt --all -- --check
```

### 2. Test-First User Story Implementation
- **Choose user stories** from `ai_assisted_development/ProductOverview.md`
- **Write failing tests** for acceptance criteria FIRST
- **Implement minimum code** to pass tests
- **Enhance** to fully meet acceptance criteria
- **Test manually** against user scenarios

### 3. Continuous Quality Assurance
```bash
# After EVERY significant change:
cargo clippy --all-targets --all-features -- -D warnings  # Must be clean
cargo test --all                                          # Must pass
cargo fmt --all                                          # Auto-format

# Before ANY commit:
npm run tauri dev        # Must run successfully
# Manual test of implemented user story acceptance criteria
```

### 4. Handle Blockers with Testing
If you encounter a blocker:
1. **Write test that reproduces the issue**
2. **Document in `ai_assisted_development/ISSUES.md`** with user story context
3. **Implement workaround with tests** if possible
4. **Mark with TODO and test** for future resolution
5. **Continue with alternative user stories**

## 📊 MANDATORY Quality Metrics

### Session Completion Requirements
**ALL MUST BE TRUE BEFORE ENDING SESSION:**

```bash
# Code Quality (Zero Tolerance)
[ ] cargo clippy --all-targets --all-features -- -D warnings  # ZERO warnings
[ ] cargo test --all                                          # 100% pass rate
[ ] cargo fmt --all                                          # Consistent formatting

# Functionality 
[ ] npm run tauri dev                                        # App runs successfully
[ ] Manual testing completed against acceptance criteria     # User story validation
[ ] All implemented features work as specified              # No broken functionality

# Test Coverage (New Standard)
[ ] New code has ≥80% test coverage                        # Measured, not estimated
[ ] All user story acceptance criteria have tests          # Comprehensive validation
[ ] All error paths tested                                 # Robust error handling

# Documentation
[ ] User story context in all code comments               # Clear traceability
[ ] Progress files updated with test status               # Include test metrics
[ ] Issues documented with reproduction tests             # Testable problem statements
```

### Testing Framework Setup (HIGH PRIORITY)
**MUST be implemented in next session if not already done:**

#### Rust Testing Configuration
```toml
# Add to Cargo.toml [dev-dependencies]
tokio-test = "0.4"
tempfile = "3.8"
mockall = "0.12"
serial_test = "3.0"

# For test coverage measurement
[profile.test]
debug = true
```

#### Test Coverage Measurement
```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Measure coverage (target: 80%+)
cargo tarpaulin --out Html --output-dir coverage
```

#### Frontend Testing Setup (FUTURE)
```json
{
  "devDependencies": {
    "vitest": "^1.0.0",
    "jsdom": "^23.0.0",
    "@testing-library/dom": "^9.0.0"
  },
  "scripts": {
    "test": "vitest",
    "test:coverage": "vitest --coverage"
  }
}
```

## 🚨 Emergency Protocols (UPDATED)

### If Quality Gates Fail
```bash
# If clippy fails
echo "❌ MANDATORY: Fix ALL clippy warnings before proceeding"
cargo clippy --all-targets --all-features -- -D warnings
# Fix every warning - no exceptions

# If tests fail  
echo "❌ MANDATORY: ALL tests must pass before proceeding"
cargo test --all
# Fix or disable failing tests - never ignore them

# If formatting is inconsistent
cargo fmt --all
# Always run formatting - consistency is mandatory
```

### If User Story Acceptance Criteria Can't Be Met
1. **Write test that reproduces the limitation**
2. **Document in `ai_assisted_development/ISSUES.md`** with test reproduction
3. **Implement closest possible approximation with tests**
4. **Mark as technical debt with test coverage**
5. **Continue with other user stories**

### If Application Won't Compile
```bash
# Clean rebuild protocol
cargo clean
rm -rf target/
cargo check --all-targets --all-features

# If still failing
cargo update  # Update dependencies
npm install   # Refresh node modules

# Ultimate reset
git status                    # Check for uncommitted changes
git clean -fd                # Remove untracked files
cargo check --all-targets    # Start fresh
```

## 📝 Session Completion Checklist (UPDATED)

### Code Quality (ZERO TOLERANCE) ⚠️
- [ ] ✅ cargo clippy passes with ZERO warnings
- [ ] ✅ cargo test passes with 100% success rate  
- [ ] ✅ cargo fmt applied and code consistently formatted
- [ ] ✅ No compiler warnings of any kind
- [ ] ✅ Security: cargo audit passes (if configured)

### Testing Requirements (NEW MANDATORY) ⚠️
- [ ] ✅ All new code has ≥80% test coverage
- [ ] ✅ User story acceptance criteria have corresponding tests
- [ ] ✅ Error handling paths tested comprehensively
- [ ] ✅ Performance requirements validated with tests
- [ ] ✅ Integration tests pass (frontend ↔ backend)

### User Story Validation ⚠️
- [ ] ✅ Manual testing performed against ALL acceptance criteria
- [ ] ✅ Automated tests validate acceptance criteria  
- [ ] ✅ Performance requirements met (measured, not estimated)
- [ ] ✅ Error scenarios tested and handled gracefully
- [ ] ✅ User workflows function end-to-end

### Documentation & Progress Tracking ⚠️
- [ ] ✅ Code comments updated with user story references
- [ ] ✅ `ai_assisted_development/PROGRESS.md` updated with test metrics
- [ ] ✅ `ai_assisted_development/SESSION_NOTES.md` updated with testing approach
- [ ] ✅ `ai_assisted_development/ISSUES.md` updated with any test-discovered issues
- [ ] ✅ `ai_assisted_development/QUALITY_METRICS.md` updated with coverage data

### Handoff Preparation ⚠️
- [ ] ✅ All quality gates pass for next agent
- [ ] ✅ Test suite runs successfully for verification
- [ ] ✅ Clear instructions for next session with testing priorities
- [ ] ✅ Environment left in clean, compilable state
- [ ] ✅ User story priorities identified with testing requirements

## 💡 Enhanced Development Tips

### Quality-First Mindset
- **Red-Green-Refactor**: Write failing test → Make it pass → Improve code
- **Zero Warnings**: Treat warnings as errors - fix immediately
- **Test Edge Cases**: Error conditions, boundary values, network failures
- **Performance Testing**: Measure against acceptance criteria, don't guess

### Automated Quality Tools (SETUP IN NEXT SESSION)
```bash
# Set up pre-commit hooks
echo '#!/bin/sh
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo fmt --all -- --check
' > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Set up CI/CD quality pipeline (future)
# - Automated testing on commit
# - Code coverage reporting  
# - Security vulnerability scanning
# - Performance regression detection
```

### Test-Driven User Story Development
1. **Read acceptance criteria carefully**
2. **Write test that validates criteria**
3. **Watch test fail (Red)**
4. **Implement minimum code to pass (Green)**  
5. **Refactor and enhance (Refactor)**
6. **Validate manually against user scenario**
7. **Add comprehensive error/edge case tests**

## 🎯 Phase-Specific Quality Focus

### Phase 1 (Weeks 1-6): Testing Foundation
**Priority**: Establish bulletproof quality practices
- **Testing Framework**: Set up comprehensive automated testing
- **User Stories #1-11**: Implement with full test coverage
- **Quality Gates**: Establish zero-tolerance policies
- **Performance Baselines**: Measure and test all timing requirements

### Phase 2 (Weeks 7-10): Advanced Testing  
**Priority**: Sophisticated testing and monitoring
- **Integration Testing**: End-to-end user workflows
- **Performance Testing**: Automated regression detection
- **Security Testing**: Automated vulnerability scanning
- **User Experience Testing**: Automated accessibility checks

### Phase 3 (Weeks 11-13): Production Quality
**Priority**: Production-ready quality assurance
- **Load Testing**: Large dataset performance validation
- **Cross-Platform Testing**: Automated multi-OS validation
- **Security Audit**: Professional security assessment
- **User Acceptance Testing**: Real user scenario validation

Remember: **Quality is not negotiable. Every line of code must meet these standards before proceeding.** 