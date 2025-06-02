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

# Test basic compilation
cd src-tauri && cargo check
cd .. && npm run tauri dev --help
```

### 2. Context Loading
Read these files in order to understand current state and product requirements:
1. **`ProductOverview.md`** - Product vision, user stories, acceptance criteria, and UI specifications
2. **`PROGRESS.md`** - Current phase, completed tasks, next priorities
3. **`AI_AGENT_INSTRUCTIONS.md`** - Complete development guidelines (this file)
4. **`SESSION_NOTES.md`** - Detailed history of previous sessions
5. **`ISSUES.md`** - Known blockers and technical debt
6. **`QUALITY_METRICS.md`** - Current quality status and targets
7. **`ImplementationPlan.md`** - Overall project architecture and timeline

### 3. Current State Assessment
```bash
# Check what compiles
cargo check --all

# Look for any new/modified files
git status

# Review recent commits
git log --oneline -10

# Check if the app runs
npm run tauri dev
```

## 📋 Session Planning Template

Copy this template to start each session:

```markdown
# Session [N] - [DATE] - Phase [X], Week [Y]

## Pre-Session Checklist
- [ ] Read ProductOverview.md for product context and user stories
- [ ] Read all progress tracking files
- [ ] Verified development environment works
- [ ] Identified current state from git status
- [ ] Application compiles successfully

## Session Objectives
Primary objectives (must complete):
- [ ] [Specific, measurable objective 1 - reference user story #X]
- [ ] [Specific, measurable objective 2 - reference user story #Y]

Secondary objectives (if time permits):
- [ ] [Nice-to-have objective 1]
- [ ] [Nice-to-have objective 2]

## User Stories Addressed
- **User Story #X**: [Brief description from ProductOverview.md]
  - Acceptance Criteria: [List key criteria from ProductOverview.md]
- **User Story #Y**: [Brief description from ProductOverview.md]
  - Acceptance Criteria: [List key criteria from ProductOverview.md]

## Success Criteria
- [ ] All primary objectives completed
- [ ] User story acceptance criteria met
- [ ] Code compiles without warnings
- [ ] Tests pass (when applicable)
- [ ] Progress files updated
- [ ] Changes tested manually against acceptance criteria

## Implementation Plan
1. **[Task 1]** (Est: X minutes) - Addresses User Story #X
   - Specific steps to complete this task
   - Expected files to modify
   - Testing approach using acceptance criteria

2. **[Task 2]** (Est: X minutes) - Addresses User Story #Y
   - Specific steps to complete this task
   - Expected files to modify
   - Testing approach using acceptance criteria

## Quality Gates
- [ ] Code follows Rust/TypeScript best practices
- [ ] All errors properly handled with PodPicoError
- [ ] Appropriate logging added
- [ ] No security vulnerabilities introduced
- [ ] Performance considerations addressed
- [ ] User story acceptance criteria validated
```

## 🎯 Development Guidelines

### User Story Driven Development

#### Always Start with User Stories
Before implementing any feature:
1. **Identify the user story** from `ProductOverview.md`
2. **Read the acceptance criteria** carefully
3. **Understand the user need** behind the feature
4. **Design implementation** to meet acceptance criteria exactly
5. **Test against acceptance criteria** not just technical specs

#### Example: User Story #1 Implementation
```rust
// User Story #1: Add new podcast subscription via RSS URL
// Acceptance Criteria:
// - Given a valid RSS feed URL, when I paste it in the add podcast dialog, 
//   then the app validates the feed within 5 seconds

#[tauri::command]
pub async fn add_podcast(rss_url: String) -> Result<Podcast, String> {
    log::info!("Adding podcast: {} (User Story #1)", rss_url);
    
    // Acceptance Criteria: Validate feed within 5 seconds
    let timeout = Duration::from_secs(5);
    let validation_result = timeout(timeout, validate_rss_feed(&rss_url)).await;
    
    match validation_result {
        Ok(Ok(podcast_info)) => {
            // Feed is valid, proceed with subscription
            let podcast = create_podcast_from_info(podcast_info).await?;
            Ok(podcast)
        },
        Ok(Err(e)) => {
            // Invalid feed - clear error message (acceptance criteria)
            Err(format!("Invalid RSS feed: {}", e))
        },
        Err(_) => {
            // Timeout - didn't validate within 5 seconds
            Err("Feed validation timed out after 5 seconds".to_string())
        }
    }
}
```

### Code Quality Standards

#### Rust Backend
```rust
// ✅ Good: Proper error handling with user story context
async fn add_podcast(rss_url: String) -> Result<Podcast, PodPicoError> {
    log::info!("Adding podcast: {} (User Story #1)", rss_url);
    
    // Validate input (User Story #1 acceptance criteria)
    if rss_url.trim().is_empty() {
        return Err(PodPicoError::InvalidRssUrl("Empty URL".to_string()));
    }
    
    // Implementation
    Ok(podcast)
}

// ❌ Bad: No reference to user story or acceptance criteria
fn add_podcast_bad(rss_url: String) -> Podcast {
    rss_url.parse().unwrap() // Can panic!
}
```

#### Error Handling Pattern
```rust
// Always use this pattern for external operations
// Include user story context in error messages where appropriate
match some_operation().await {
    Ok(result) => {
        log::info!("Operation succeeded for User Story #X");
        result
    },
    Err(e) => {
        log::error!("Operation failed for User Story #X: {}", e);
        return Err(PodPicoError::from(e));
    }
}
```

#### Logging Standards
```rust
// Use appropriate log levels and include user story context
log::debug!("Detailed debugging info for User Story #X: {:?}", data);
log::info!("User Story #X: Normal operation: {}", operation);
log::warn!("User Story #X: Unusual but recoverable: {}", warning);
log::error!("User Story #X failed: {}", error);
```

### Testing Requirements

#### User Story Validation Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test based on User Story #1 acceptance criteria
    #[tokio::test]
    async fn test_user_story_1_valid_rss_feed() {
        // Acceptance Criteria: Given a valid RSS feed URL, when I paste it 
        // in the add podcast dialog, then the app validates the feed within 5 seconds
        
        // Arrange
        let url = "https://example.com/valid-feed.xml".to_string();
        let start_time = std::time::Instant::now();
        
        // Act
        let result = add_podcast(url).await;
        let elapsed = start_time.elapsed();
        
        // Assert
        assert!(result.is_ok(), "Should accept valid RSS feed");
        assert!(elapsed < Duration::from_secs(5), "Should validate within 5 seconds");
    }

    #[tokio::test]
    async fn test_user_story_1_invalid_url_clear_error() {
        // Acceptance Criteria: Given an invalid URL, when I attempt to subscribe, 
        // then I receive a clear error message explaining the issue
        
        // Arrange
        let url = "invalid-url".to_string();
        
        // Act
        let result = add_podcast(url).await;
        
        // Assert
        assert!(result.is_err(), "Should reject invalid URL");
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.is_empty(), "Should provide clear error message");
        assert!(error_msg.contains("invalid") || error_msg.contains("Invalid"), 
                "Error message should explain the issue clearly");
    }
}
```

### Frontend Guidelines

#### Component Structure with User Story Context
```typescript
// User Story #2: View all episodes of a specific podcast
// Acceptance Criteria: Given I select a podcast folder, when the episode list loads, 
// then all episodes for that podcast are displayed within 3 seconds

const PodcastEpisodeList: React.FC<{ podcastId: number }> = ({ podcastId }) => {
    const [episodes, setEpisodes] = useState<Episode[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [loadStartTime, setLoadStartTime] = useState<number>(0);

    useEffect(() => {
        loadEpisodes();
    }, [podcastId]);

    const loadEpisodes = async () => {
        try {
            setLoading(true);
            const startTime = Date.now();
            setLoadStartTime(startTime);
            
            // User Story #2: Load episodes for specific podcast
            const result = await invoke('get_episodes', { podcast_id: podcastId });
            
            const loadTime = Date.now() - startTime;
            console.log(`User Story #2: Episodes loaded in ${loadTime}ms`);
            
            // Acceptance Criteria: Should display within 3 seconds
            if (loadTime > 3000) {
                console.warn(`User Story #2: Load time exceeded 3 seconds (${loadTime}ms)`);
            }
            
            setEpisodes(result);
        } catch (err) {
            setError(`Failed to load episodes: ${err}`);
            console.error('User Story #2 failed:', err);
        } finally {
            setLoading(false);
        }
    };

    // User Story #2: Show episodes with title, date, duration, and status icon
    if (loading) return <div>Loading episodes...</div>;
    if (error) return <div>Error: {error}</div>;
    
    return (
        <div className="episode-list">
            {episodes.map(episode => (
                <EpisodeItem 
                    key={episode.id} 
                    episode={episode}
                    // User Story #2: Each episode shows title, date, duration, and status icon
                    showTitle={true}
                    showDate={true}
                    showDuration={true}
                    showStatusIcon={true}
                />
            ))}
        </div>
    );
};
```

## 🔄 Session Workflow

### 1. Start with User Stories
- **Choose user stories** from `ProductOverview.md` that align with current phase
- **Read acceptance criteria** carefully before starting implementation
- **Break down user stories** into implementable technical tasks
- **Test against acceptance criteria** not just technical specs

### 2. Test Frequently
```bash
# After each significant change:
cargo check              # Quick syntax check
cargo test               # Run tests (include user story validation tests)
npm run tauri dev        # Verify app still runs
# Manual testing against acceptance criteria
```

### 3. Document as You Go
- **Reference user stories** in commit messages and code comments
- **Update TODO comments** when completing user story features
- **Add new TODO comments** for future work discovered
- **Update progress files** with user story completion status

### 4. Handle Blockers Gracefully
If you encounter a blocker:
1. **Document it immediately** in `ISSUES.md` with user story context
2. **Implement a workaround** if possible that still meets acceptance criteria
3. **Mark with TODO** for future resolution with user story reference
4. **Continue with alternative user stories**
5. **Don't get stuck** - move to next priority user story

## 📊 Progress Tracking

### Required Updates Every Session
1. **`PROGRESS.md`**
   - Update completion status of current tasks **with user story references**
   - Mark completed items with ✅ and note which user stories were addressed
   - Update "Last Updated" date
   - Add session to history with user story achievements

2. **`SESSION_NOTES.md`**
   - Add detailed session log **with user story context**
   - Document issues encountered and resolutions **for specific user stories**
   - Note any architectural decisions made **and their impact on user stories**
   - Record lessons learned **about user story implementation**

3. **`ISSUES.md`**
   - Add any new issues discovered **with user story impact assessment**
   - Update status of existing issues
   - Move resolved issues to resolved section **with user story completion notes**

4. **`QUALITY_METRICS.md`**
   - Update compilation status
   - Record any quality improvements **that affect user story delivery**
   - Update test coverage if tests added **for user story validation**

## 🛠️ Common Development Tasks

### Setting Up Database Schema (User Story Driven)
```rust
// User Stories #1, #2, #3, #4: Podcast and Episode Management
// These user stories require podcast and episode data storage

pub async fn initialize(&self) -> Result<(), PodPicoError> {
    log::info!("Creating database tables for podcast management user stories");
    
    // User Story #1: Add podcast subscription
    // User Story #4: Remove podcast subscription
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS podcasts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            rss_url TEXT UNIQUE NOT NULL,
            description TEXT,
            artwork_url TEXT,
            website_url TEXT,
            last_updated DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    "#)
    .execute(&self.pool)
    .await?;
    
    // User Story #2: View episodes of specific podcast
    // User Story #3: Download episodes
    // User Story #5: Mark episodes as listened
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS episodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            podcast_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            episode_url TEXT NOT NULL,
            published_date DATETIME,
            duration INTEGER,
            file_size INTEGER,
            local_file_path TEXT,
            status TEXT CHECK(status IN ('new', 'unlistened', 'listened')) DEFAULT 'new',
            downloaded BOOLEAN DEFAULT FALSE,
            on_device BOOLEAN DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (podcast_id) REFERENCES podcasts (id) ON DELETE CASCADE
        )
    "#)
    .execute(&self.pool)
    .await?;
    
    Ok(())
}
```

### Adding New Tauri Commands (User Story Focused)
1. **Identify user story** the command addresses
2. **Define function in `commands.rs`** with user story context
3. **Add to command handler in `lib.rs`**
4. **Test from frontend** against acceptance criteria
5. **Validate acceptance criteria** are met

### Creating Frontend Components (UI/UX from ProductOverview)
Reference the detailed UI specifications in `ProductOverview.md`:
1. **Follow 3-pane layout** as specified in ProductOverview.md
2. **Implement email-app inspired navigation** (left sidebar, episode list, details)
3. **Use specified interaction patterns** (Combined Inbox, podcast folders, etc.)
4. **Test against UI acceptance criteria** from user stories

## 🚨 Emergency Protocols

### If User Story Acceptance Criteria Can't Be Met
1. **Document the limitation** in `ISSUES.md` with user story reference
2. **Implement closest possible approximation** that still provides user value
3. **Mark as technical debt** for future improvement
4. **Continue with other user stories** to maintain progress
5. **Note impact on overall user experience**

### If Compilation Fails
```bash
# Clean and rebuild
cargo clean
npm install
cargo check

# Check for dependency issues
cargo update
```

### If Tests Fail
1. **Don't ignore failing tests**, especially user story validation tests
2. **Fix the test or fix the code**
3. **Document why** if test needs to be changed (user story evolution)
4. **Add more tests** to prevent regression of user story functionality

### If Application Won't Start
1. **Check `cargo check`** for compilation errors
2. **Verify all dependencies installed**
3. **Check for database connection issues** (affects many user stories)
4. **Review logs** for error messages with user story context

## 📝 Session Completion Checklist

Before ending a session:

### Code Quality
- [ ] Code compiles without warnings
- [ ] All new functions have error handling
- [ ] Appropriate logging added **with user story context**
- [ ] No obvious security issues
- [ ] User story acceptance criteria addressed

### Testing
- [ ] Manual testing performed **against acceptance criteria**
- [ ] Unit tests added (when framework ready) **for user story validation**
- [ ] Integration tested (frontend <-> backend) **for complete user workflows**

### Documentation
- [ ] Code comments updated **with user story references**
- [ ] TODO items documented **with user story context**
- [ ] Architecture decisions recorded **with user story impact**

### Progress Tracking
- [ ] `PROGRESS.md` updated **with user story completion status**
- [ ] `SESSION_NOTES.md` updated **with user story achievements**
- [ ] `ISSUES.md` updated **with any user story blockers**
- [ ] `QUALITY_METRICS.md` updated if applicable

### Handoff Preparation
- [ ] Clear instructions for next session **with user story priorities**
- [ ] Priority user stories identified for next session
- [ ] Blockers documented **with user story impact**
- [ ] Environment left in clean state

## 🎯 Phase-Specific Focus Areas

### Phase 1 (Weeks 1-6): MVP Core
**Priority**: Get basic user stories working (User Stories #1-11)
- Focus on **User Stories #1-4** (podcast management) first
- Implement **User Stories #5-7** (episode status management)
- Create **User Stories #8-11** (USB device integration)
- Test **core user workflows** frequently against acceptance criteria

### Phase 2 (Weeks 7-10): Enhanced Features
**Priority**: User experience and polish (User Stories #12-15, Future #16-18)
- Add **User Story #12** (search functionality)
- Implement **User Stories #13-15** (UI improvements)
- Focus on **performance and usability**
- Add **progress indicators** and batch operations

### Phase 3 (Weeks 11-13): Quality & Distribution
**Priority**: Production readiness
- Comprehensive testing **of all user stories**
- Performance optimization **for user story scenarios**
- Cross-platform compatibility **testing**
- Documentation completion **including user manual**

## 💡 Tips for Success

### User Story Focus
- **Always ask "why"** - understand the user need behind each story
- **Test with real usage scenarios** not just technical edge cases
- **Consider user experience** over technical elegance
- **Validate acceptance criteria** are realistic and achievable

### Time Management
- **Estimate user stories** conservatively based on acceptance criteria complexity
- **Focus on primary user stories** before nice-to-have features
- **Don't over-engineer** - meet acceptance criteria, don't exceed them unnecessarily
- **Leave time for user story validation** and testing

### Problem Solving
- **Read acceptance criteria carefully** before implementing
- **Use user story context** for debugging and troubleshooting
- **Test assumptions** against real user scenarios
- **Ask "does this solve the user's problem?"** frequently

### Quality Focus
- **Working user stories** are better than perfect code
- **User story completion** drives technical decisions
- **Performance matters** for user experience (acceptance criteria timing)
- **User feedback** (through acceptance criteria) drives priorities

## 📚 Reference Resources

### Product Context
- **`ProductOverview.md`** - Complete product vision and user stories
- **User Story Acceptance Criteria** - Detailed requirements for each feature

### Tauri Documentation
- [Tauri Commands](https://tauri.app/develop/calling-rust/)
- [Frontend Integration](https://tauri.app/develop/frontend/)

### Rust Resources
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [SQLx Documentation](https://docs.rs/sqlx/)

### Frontend Resources
- [React TypeScript](https://react-typescript-cheatsheet.netlify.app/)
- [Tauri API Reference](https://tauri.app/api/js/)

## 🎯 User Story Quick Reference

### Core MVP User Stories (Phase 1 Priority)
1. **#1**: Add podcast subscription via RSS URL
2. **#2**: View all episodes of specific podcast  
3. **#3**: Download episodes from specific podcast
4. **#4**: Remove podcast subscriptions
5. **#5**: Mark episodes as "listened"
6. **#6**: See episode status within each podcast
7. **#7**: View all new episodes across podcasts (Combined Inbox)
8. **#8**: See USB device storage capacity
9. **#9**: Transfer episodes to USB device
10. **#10**: Remove episodes from USB device
11. **#11**: See which episodes are on USB device

### UI/UX User Stories (Phase 2 Priority)
12. **#12**: Search for episodes within podcast
13. **#13**: See clear status indicators
14. **#14**: Sort episodes by date within podcast
15. **#15**: Filter episodes by status within podcast

Remember: **Every line of code should serve a user story. Every feature should meet acceptance criteria. The user's needs drive all technical decisions.** 