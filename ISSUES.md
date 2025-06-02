# PodPico Known Issues and Blockers

This file tracks all known issues, blockers, and technical debt that need to be addressed during development.

## Current Blockers

*No active blockers at this time.*

## Known Issues

### Issue #1: USB Device Detection API Uncertainty
- **Category**: Technical Debt
- **Priority**: Medium
- **Module**: `usb_manager.rs`
- **Description**: The sysinfo library API has changed, and the current implementation stub may not work for actual USB device detection.
- **Impact**: USB device detection feature may need API research and testing
- **Workaround**: None needed yet (feature not implemented)
- **Next Steps**: Research sysinfo v0.32 API for disk/device detection when implementing USB features
- **Assigned Session**: Week 5-6 (USB Integration phase)

### Issue #2: RSS Feed Validation Strategy Undefined
- **Category**: Design Decision
- **Priority**: Medium  
- **Module**: `rss_manager.rs`
- **Description**: Need to define how strictly to validate RSS feeds and handle malformed/non-standard feeds
- **Impact**: May reject valid but non-standard podcast feeds
- **Workaround**: Start with basic validation, expand as needed
- **Next Steps**: Research common RSS feed variations and validation libraries
- **Assigned Session**: Week 3-4 (Podcast Management phase)

## Resolved Issues

### Issue #1: Tauri Feature Configuration ✅ RESOLVED
- **Resolution Date**: 2025-05-31
- **Problem**: Invalid `shell-open` feature flag in Tauri v2
- **Solution**: Removed feature flag, functionality available via default configuration
- **Resolved By**: Session 0 setup

### Issue #2: Sysinfo API Compatibility ✅ RESOLVED
- **Resolution Date**: 2025-05-31
- **Problem**: `SystemExt` and `DiskExt` traits removed in newer sysinfo versions
- **Solution**: Updated imports to use simplified `System` API
- **Resolved By**: Session 0 setup

## Technical Debt

### High Priority Debt
1. **Database Schema Implementation**
   - **Location**: `src-tauri/src/database.rs`
   - **Description**: Complete database schema needs to be implemented from design
   - **Effort**: 1-2 hours
   - **Blocking**: All podcast management features

2. **Error Handling Integration**
   - **Location**: All backend modules
   - **Description**: Replace generic errors with specific error types
   - **Effort**: Ongoing during implementation
   - **Blocking**: Proper error reporting to users

### Medium Priority Debt
1. **Frontend Architecture**
   - **Location**: `src/` directory
   - **Description**: Default React template needs complete redesign for 3-pane layout
   - **Effort**: 4-6 hours
   - **Blocking**: User interface implementation

2. **Testing Framework**
   - **Location**: Project-wide
   - **Description**: No testing framework set up for Rust backend or React frontend
   - **Effort**: 2-3 hours setup
   - **Blocking**: Quality assurance and regression prevention

### Low Priority Debt
1. **Configuration Management**
   - **Location**: `src-tauri/src/config.rs`
   - **Description**: File-based configuration system needs implementation
   - **Effort**: 2-3 hours
   - **Blocking**: User customization features

2. **Logging Integration**
   - **Location**: All modules
   - **Description**: Structured logging needs to be added throughout application
   - **Effort**: Ongoing during implementation
   - **Blocking**: Debugging and troubleshooting

## Future Considerations

### Performance Concerns
1. **Large Episode Lists**
   - **Issue**: Frontend may become sluggish with thousands of episodes
   - **Solution**: Implement virtual scrolling or pagination
   - **Timeline**: Phase 2 (UI Polish)

2. **Concurrent Downloads**
   - **Issue**: Multiple simultaneous downloads may overwhelm system
   - **Solution**: Implement download queue with concurrency limits
   - **Timeline**: Phase 1 (File Operations)

### Security Considerations
1. **RSS Feed Validation**
   - **Issue**: Malicious RSS feeds could potentially exploit parser
   - **Solution**: Strict input validation and sandboxed parsing
   - **Timeline**: Phase 1 (RSS Implementation)

2. **File System Access**
   - **Issue**: USB device operations require careful path validation
   - **Solution**: Strict path validation and user confirmation
   - **Timeline**: Phase 1 (USB Integration)

### Platform Compatibility
1. **USB Device Detection**
   - **Issue**: Different USB mounting behavior across Linux/Windows/macOS
   - **Solution**: Platform-specific detection logic
   - **Timeline**: Phase 1 (USB Integration)

2. **File Path Handling**
   - **Issue**: Path separators and conventions differ across platforms
   - **Solution**: Use standard Rust path handling libraries
   - **Timeline**: Ongoing during file operations implementation

## Issue Template

When adding new issues, use this template:

```markdown
### Issue #X: [Brief Description]
- **Category**: [Bug/Enhancement/Technical Debt/Design Decision]
- **Priority**: [High/Medium/Low]
- **Module**: [Affected module/file]
- **Description**: [Detailed description of the issue]
- **Impact**: [How this affects development or users]
- **Workaround**: [Temporary solution if available]
- **Next Steps**: [What needs to be done to resolve]
- **Assigned Session**: [When this should be addressed]
```

## Issue Tracking Guidelines

### Priority Levels
- **High**: Blocks critical functionality or security issue
- **Medium**: Affects user experience or development efficiency
- **Low**: Nice-to-have improvements or optimizations

### Categories
- **Bug**: Something that doesn't work as intended
- **Enhancement**: New feature or improvement request
- **Technical Debt**: Code quality or maintainability issue
- **Design Decision**: Architecture or approach needs to be decided

### Resolution Process
1. Issues should be addressed in priority order
2. High priority issues should be resolved before moving to new features
3. All resolutions should be tested and documented
4. Resolved issues should be moved to "Resolved Issues" section with solution details 