# AI Agent Development Instructions

You are an AI agent taking over a PodPico development session. Follow these comprehensive instructions to ensure high-quality, test-driven development that meets all product requirements and maintains code excellence.

## 🚀 MANDATORY Session Startup Protocol

### 1. Get Current Date and Time
Before starting any work, get the current session date:

```bash
# Get current date for session tracking
SESSION_DATE=$(date +"%Y-%m-%d")
SESSION_TIME=$(date +"%H:%M:%S")
echo "Starting PodPico development session on $SESSION_DATE at $SESSION_TIME"
```

### 2. Environment Verification
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

### 3. Context Loading
Read these files in order to understand current state and product requirements:
1. **`ai_assisted_development/ProductOverview.md`** - Product vision, user stories, acceptance criteria, and UI specifications
2. **`ai_assisted_development/PROGRESS.md`** - Current phase, completed tasks, next priorities
3. **`ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`** - Complete development guidelines (this file)
4. **`ai_assisted_development/SESSION_NOTES.md`** - Detailed history of previous sessions
5. **`ai_assisted_development/ISSUES.md`** - Known blockers and technical debt
6. **`ai_assisted_development/QUALITY_METRICS.md`** - Current quality status and targets
7. **`ai_assisted_development/ImplementationPlan.md`** - Overall project architecture and timeline

### 4. Current State Assessment
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

Copy this template to start each session and populate with current date:

```bash
# Get session information
SESSION_DATE=$(date +"%Y-%m-%d")
SESSION_TIME=$(date +"%H:%M:%S")
SESSION_NUMBER=$(git rev-list --count HEAD)  # Use commit count as session number
```

```markdown
# Session ${SESSION_NUMBER} - ${SESSION_DATE} ${SESSION_TIME} - Phase [X], Week [Y]

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

### Test Naming Quick Reference ⚠️ MANDATORY
```rust
// PRIMARY: User Story Tests
test_user_story_X_[brief_description]()
test_user_story_X_[specific_acceptance_criteria]()
test_user_story_X_performance_requirements()
test_user_story_X_complete_workflow()

// SECONDARY: Component/Error Tests  
test_[component]_[functionality]()
test_[component]_[error_condition]()
```

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

### Date and Time Management (MANDATORY)

**ALWAYS use system commands to get current date/time information. NEVER hardcode dates.**

#### Standard Date/Time Commands
```bash
# Basic date information
SESSION_DATE=$(date +"%Y-%m-%d")          # 2024-01-15
SESSION_TIME=$(date +"%H:%M:%S")          # 14:30:22
SESSION_DATETIME=$(date +"%Y-%m-%d %H:%M:%S")  # 2024-01-15 14:30:22

# Session tracking
SESSION_NUMBER=$(git rev-list --count HEAD)  # Use commit count
UNIX_TIMESTAMP=$(date +%s)                   # Unix timestamp

# For logging and documentation
LOG_TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S UTC" --utc)
```

#### Usage Examples
```rust
// In Rust code - use chrono for timestamps
log::info!("Operation started - {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));

// In documentation updates
// Instead of: "Updated on [DATE]"
// Use: "Updated on $(date +"%Y-%m-%d %H:%M:%S")"
```

#### For All Documentation Updates
Before updating any `.md` files, always execute:
```bash
DOC_UPDATE_TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
echo "Documentation updated: $DOC_UPDATE_TIMESTAMP"
```

### Test-Driven Development (MANDATORY)

#### Test Function Naming Conventions (MANDATORY)
**ALL test functions MUST follow these naming patterns for traceability:**

📋 **QUICK REFERENCE**: See `ai_assisted_development/TEST_NAMING_CONVENTIONS.md` for comprehensive examples and decision flowchart.

```rust
// PRIMARY PATTERN: User Story Acceptance Criteria Tests
#[tokio::test]
async fn test_user_story_X_[brief_description]() {
    // Tests core acceptance criteria for User Story #X
}

#[tokio::test] 
async fn test_user_story_X_[specific_acceptance_criteria]() {
    // Tests specific acceptance criteria requirement
}

// EXAMPLES FROM EXISTING CODEBASE:
test_user_story_1_add_podcast_command()                    // Core functionality
test_user_story_1_acceptance_criteria_complete()           // Full acceptance criteria
test_user_story_3_download_progress_tracking()            // Specific requirement
test_user_story_8_performance_requirements()              // Performance criteria
test_user_story_10_remove_episode_database_integration()  // Integration aspect

// SECONDARY PATTERNS: Component and Error Testing
#[tokio::test]
async fn test_[component]_[functionality]() {
    // Component-specific functionality tests
}

#[tokio::test]
async fn test_[component]_[error_condition]() {
    // Error handling and edge case tests
}

// EXAMPLES:
test_database_initialization()                 // Component setup
test_add_podcast_invalid_url()                // Error condition
test_file_manager_disk_space_check()          // Component functionality
test_usb_manager_nonexistent_device()         // Error handling

// PERFORMANCE TESTING PATTERN:
#[tokio::test]
async fn test_user_story_X_performance_requirements() {
    // Validates timing requirements from acceptance criteria
    let start_time = std::time::Instant::now();
    // ... test implementation
    let elapsed = start_time.elapsed();
    assert!(elapsed < Duration::from_secs(5), "Should complete within 5 seconds");
}

// INTEGRATION TESTING PATTERN:
#[tokio::test]
async fn test_user_story_X_complete_workflow() {
    // End-to-end testing of entire user story workflow
}
```

#### Test Organization Structure (MANDATORY)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // GROUP 1: User Story Acceptance Criteria Tests (PRIORITY)
    #[tokio::test]
    async fn test_user_story_X_[primary_acceptance_criteria]() { }
    
    #[tokio::test] 
    async fn test_user_story_X_[secondary_acceptance_criteria]() { }
    
    #[tokio::test]
    async fn test_user_story_X_performance_requirements() { }
    
    // GROUP 2: Component Functionality Tests
    #[tokio::test]
    async fn test_[component]_[core_functionality]() { }
    
    // GROUP 3: Error Handling and Edge Cases
    #[tokio::test]
    async fn test_[component]_[error_condition]() { }
    
    // GROUP 4: Integration and Workflow Tests
    #[tokio::test]
    async fn test_user_story_X_complete_workflow() { }
}
```

#### Test Documentation Requirements (MANDATORY)
```rust
#[tokio::test]
async fn test_user_story_3_download_progress_tracking() {
    // PURPOSE: Validates User Story #3 Acceptance Criteria
    // CRITERIA: "User can see download progress percentage during episode download"
    // TIMING: Progress updates must be received within 1 second
    // ERROR CASES: Network interruption, invalid URLs, insufficient disk space
    
    let episode_id = create_test_episode().await;
    let mut progress_updates = Vec::new();
    
    // Test implementation...
    
    // ASSERTIONS: Link back to acceptance criteria
    assert!(!progress_updates.is_empty(), "User Story #3: Progress updates required");
    assert!(progress_updates.last().unwrap() >= &100.0, "User Story #3: Must reach 100%");
}
```

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
    log::info!("Adding podcast: {} (User Story #1) - Session: {}", rss_url, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
    
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
            log::info!("User Story #1 completed successfully: {} - {}", podcast.name, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
            Ok(podcast)
        },
        Ok(Err(e)) => {
            log::warn!("User Story #1 validation failed: {} - {}", e, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
            Err(PodPicoError::InvalidRssUrl(format!("Invalid RSS feed: {}", e)))
        },
        Err(_) => {
            log::error!("User Story #1 timeout: Feed validation exceeded 5 seconds - {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
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

### Testing Framework Setup (COMPLETED ✅)
**Comprehensive test coverage reporting is now integrated and ready for use:**

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

#### Test Coverage Integration (IMPLEMENTED ✅)
**cargo-tarpaulin is installed and fully configured with automation:**

```bash
# AUTOMATED COVERAGE COMMANDS (Ready to Use)
make coverage           # Development mode - generate coverage report
make coverage-open      # Generate coverage + open HTML report in browser
make coverage-ci        # CI mode - strict 80% minimum, fails build if below target
make coverage-custom TARGET=85  # Custom coverage target

# DIRECT TARPAULIN COMMANDS
cargo tarpaulin --config podpico     # Development configuration
cargo tarpaulin --config ci          # CI/CD configuration
./src-tauri/scripts/coverage.sh      # Comprehensive coverage script

# QUALITY INTEGRATION
make quality-full       # Includes coverage analysis in quality pipeline
make session-end        # End-of-session protocol includes coverage
make ci-check          # Complete CI/CD pipeline with coverage gate
```

#### Coverage Configuration Files (CREATED ✅)
- **`src-tauri/tarpaulin.toml`** - Comprehensive tarpaulin configuration
- **`src-tauri/scripts/coverage.sh`** - Automated coverage analysis script  
- **`src-tauri/Makefile`** - Quality-first development automation
- **`src-tauri/coverage/`** - Generated coverage reports directory

#### Coverage Features Implemented
- **Multiple Output Formats**: HTML (detailed), XML (CI/CD), Stdout (console)
- **Quality Gate Integration**: Enforces 80% minimum coverage in CI mode
- **Automated Pre-checks**: Runs clippy, formatting, and compilation before coverage
- **Performance Monitoring**: Tracks coverage analysis duration and trends
- **Error Handling**: Graceful handling of coverage failures with actionable feedback
- **Documentation Integration**: Updates quality metrics with coverage data

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

Before updating any documentation files, get current session information:
```bash
# Get documentation update variables
DOC_UPDATE_DATE=$(date +"%Y-%m-%d")
DOC_UPDATE_TIME=$(date +"%H:%M:%S")
SESSION_NUMBER=$(git rev-list --count HEAD)
```

- [ ] ✅ Code comments updated with user story references
- [ ] ✅ `ai_assisted_development/PROGRESS.md` updated with test metrics and session date
- [ ] ✅ `ai_assisted_development/SESSION_NOTES.md` updated with testing approach and session timestamp
- [ ] ✅ `ai_assisted_development/ISSUES.md` updated with any test-discovered issues and discovery date
- [ ] ✅ `ai_assisted_development/QUALITY_METRICS.md` updated with coverage data and measurement date
- [ ] ✅ **`README.md` updated with current project status (if applicable)**

## 🚨 MANDATORY: GIT COMMIT MESSAGE CREATION

### **BEFORE ENDING ANY SESSION** ⚠️ CRITICAL

Every AI agent session MUST end with creating a comprehensive commit message. This is NOT optional and should be treated as the final deliverable of each session.

#### Commit Message Template (MANDATORY)

First, get the current session information:
```bash
# Get commit message variables
SESSION_DATE=$(date +"%Y-%m-%d")
SESSION_TIME=$(date +"%H:%M:%S")
SESSION_NUMBER=$(git rev-list --count HEAD)
COMMIT_TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
```

Then use this template:
```
feat: Session ${SESSION_NUMBER} - [Primary User Story] - [Brief Description]

Implements User Story #[X]: [User Story Title]
- Acceptance Criteria: [List key criteria met]
- Performance: [Performance achievements vs requirements]
- Testing: [Test coverage added/maintained]

Technical Implementation:
- [Key module/component changes]
- [Architecture improvements]
- [Quality metrics achieved]

Quality Assurance:
- Tests: [before/after test count] ([percentage]% pass rate)
- Clippy: Zero warnings maintained
- Performance: [Key performance metrics]

Addresses: [List any issues resolved]
Next: [Next session priorities]

Session: ${SESSION_DATE} ${SESSION_TIME}
Co-authored-by: AI Agent Session ${SESSION_NUMBER} <${COMMIT_TIMESTAMP}>
```

#### Commit Message Guidelines ⚠️ MANDATORY
- **Type**: Use conventional commits (feat:, fix:, docs:, test:, refactor:)
- **Session Context**: Always include session number and primary focus
- **User Story Focus**: Lead with the main user story addressed
- **Acceptance Criteria**: Explicitly list acceptance criteria met
- **Technical Details**: Include key implementation details
- **Quality Metrics**: Include test counts and performance data
- **Forward Looking**: Mention next session priorities
- **AI Attribution**: Credit the AI agent session in co-authored-by

#### When to Create Commit Message
- **Timing**: After all quality gates pass but BEFORE ending session
- **Purpose**: Documents session achievements for next agent
- **Format**: Ready-to-use git commit message
- **Validation**: Must reflect actual work completed in session

### Session Reflection and Improvement Suggestions ⚠️ MANDATORY

Before ending the session, reflect on the development process and suggest improvements:

```bash
# Get session reflection timestamp
REFLECTION_TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
echo "Session reflection and improvement analysis: $REFLECTION_TIMESTAMP"
```

**MANDATORY: Provide improvement suggestions to the user covering:**

#### 1. **AI Agent Instructions Improvements**
Reflect on this session and suggest specific improvements to `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`:
- Were any instructions unclear or missing?
- What additional quality gates or checks would be helpful?
- Were there any development patterns that should be standardized?
- Did any part of the workflow cause inefficiency or confusion?
- What additional automation or tooling could improve the process?

#### 2. **Documentation Structure Improvements**
Suggest improvements to the overall documentation system:
- Are there missing documents that would help future agents?
- Should any existing documents be restructured or merged?
- What information was hard to find or access?
- Are there redundancies that could be eliminated?

#### 3. **Development Process Improvements**
Identify process improvements based on this session:
- What slowed down development unnecessarily?
- Which quality gates were most valuable vs. least valuable?
- What testing strategies worked best?
- Where could parallel work be better utilized?
- What manual steps could be automated?

#### 4. **User Story and Acceptance Criteria Improvements**
Suggest improvements to product documentation:
- Were any user stories unclear or incomplete?
- Did acceptance criteria need clarification?
- What additional context would help future development?
- Are there missing user stories or edge cases to consider?

#### 5. **Tooling and Environment Improvements**
Recommend tooling or environment enhancements:
- What development tools would improve efficiency?
- Are there missing linting rules or quality checks?
- What IDE configurations or extensions would help?
- Should any build scripts or automation be added?

**Present these suggestions in a clear, actionable format to the user before ending the session.**

#### Improvement Suggestion Template
Use this format when presenting suggestions to the user:

```markdown
## 🔄 Session Improvement Suggestions - ${REFLECTION_TIMESTAMP}

Based on this development session, here are my recommendations for improving the AI agent development process:

### 🤖 AI Agent Instructions (`ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`)
**Priority: [High/Medium/Low]**
- **Issue**: [Specific problem encountered]
- **Suggestion**: [Specific improvement recommendation]
- **Benefit**: [How this would improve future sessions]

### 📚 Documentation Structure
**Priority: [High/Medium/Low]**
- **Issue**: [Documentation gap or problem]
- **Suggestion**: [Specific documentation improvement]
- **Benefit**: [How this would help future agents]

### ⚙️ Development Process
**Priority: [High/Medium/Low]**  
- **Issue**: [Process inefficiency or problem]
- **Suggestion**: [Specific process improvement]
- **Benefit**: [Expected efficiency or quality gain]

### 📋 User Stories & Acceptance Criteria
**Priority: [High/Medium/Low]**
- **Issue**: [Clarity or completeness problem]
- **Suggestion**: [Specific improvement to product docs]
- **Benefit**: [How this would improve development]

### 🛠️ Tooling & Environment
**Priority: [High/Medium/Low]**
- **Issue**: [Missing tool or configuration problem]
- **Suggestion**: [Specific tool or environment improvement]
- **Benefit**: [Expected productivity or quality improvement]

### 📈 Metrics & Observations
- **Session Duration**: [Actual time spent]
- **Quality Gates**: [Which were most/least valuable]
- **Blockers Encountered**: [List any significant obstacles]
- **Testing Effectiveness**: [How well TDD approach worked]
- **User Story Completion**: [Rate of progress against acceptance criteria]

### 🎯 Top 3 Recommendations
1. **[Highest priority improvement]** - [Expected impact]
2. **[Second priority improvement]** - [Expected impact] 
3. **[Third priority improvement]** - [Expected impact]
```

### Handoff Preparation ⚠️
- [ ] ✅ All quality gates pass for next agent
- [ ] ✅ Test suite runs successfully for verification
- [ ] ✅ Clear instructions for next session with testing priorities
- [ ] ✅ Environment left in clean, compilable state
- [ ] ✅ User story priorities identified with testing requirements
- [ ] ✅ **🚨 SESSION REFLECTION AND IMPROVEMENT SUGGESTIONS PROVIDED TO USER 🚨**
- [ ] ✅ **🚨 COMMIT MESSAGE CRAFTED AND READY FOR APPLICATION 🚨**
- [ ] ✅ **README.md updated with current status (see README Update Guidelines below)**

## 📄 README.md Update Guidelines

### When to Update README.md
Update the README.md file when any of the following occur during a session:
- **Phase or Week progression** (e.g., completing Week 3-4, moving to Week 5-6)
- **Significant milestone completion** (e.g., multiple user stories completed)
- **Overall progress percentage changes** significantly (±10% or more)
- **Architecture changes** that affect the technical stack
- **New major features** that should be highlighted in the feature list

### What to Update in README.md

#### 1. Project Status Section
```markdown
## 📋 Project Status

- **Phase**: [Current Phase Number] ([Phase Name])
- **Week**: [Current Week Range] ([Week Description]) [Status: ✅ COMPLETED / 🔄 IN PROGRESS / ⏳ PENDING]
- **Overall Progress**: [Percentage]% ([Brief description of current state])
- **Next Priority**: [Next major user story or milestone]
```

#### 2. Development Phases Section
Mark completed phases/weeks with ✅ and update status:
```markdown
### Phase 1: MVP Core (Weeks 1-6)
- ✅ **Week 1-2**: Project setup and basic infrastructure (COMPLETED)
- ✅ **Week 3-4**: Podcast management (COMPLETED - User Stories #1, #2, #5, #6, #7)
- 🔄 **Week 5-6**: File operations and USB integration (IN PROGRESS - User Stories #3, #8, #9)
```

#### 3. Current Status Footer
```markdown
**Current Status**: [Brief description of what's ready for next session]
```

#### 4. Architecture Section (if applicable)
Update key features list when new major features are implemented:
```markdown
### Key Features (Implemented/Planned)
- ✅ RSS feed podcast subscription management
- ✅ Episode download and local storage
- 🔄 USB device detection and file transfer
- ✅ Episode status tracking (New/Unlistened/Listened)
- ⏳ Configurable settings and preferences
```

### README Update Examples

#### Example 1: Major Milestone Completion
If completing multiple user stories in a session:
```markdown
- **Overall Progress**: 45% (User Stories #1-9 completed with full test coverage)
- **Next Priority**: User Story #10 (Remove episodes from USB device)
```

#### Example 2: Phase Transition
If moving from one phase/week to another:
```markdown
- **Week**: 5-6 (File operations and USB integration) ✅ COMPLETED
- **Week**: 7-8 (User Interface Polish) 🔄 IN PROGRESS
```

### README Update Checklist
- [ ] Project Status section reflects current phase and progress
- [ ] Development Phases section shows completed items with ✅
- [ ] Key Features section updated with implemented features
- [ ] Current Status footer describes what's ready for next session
- [ ] Overall Progress percentage is accurate based on user stories completed

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