# Session-End Documentation Checklist

This checklist ensures all AI agent documentation is updated and consistent at the end of each development session.

## 🔍 Pre-Checklist: Run Documentation Verification
```bash
npm run doc:verify
# or
./scripts/doc-verify.sh
```

## 📝 Required Documentation Updates

### 1. SESSION_NOTES.md ✅
**Template for new session entry:**
```markdown
## Session X - YYYY-MM-DD HH:MM:SS - [Brief Session Title] ✅ [STATUS]

### Session Info
- **Phase**: [Current Phase]
- **Week**: [Current Week/Focus]
- **Duration**: [Session Duration]
- **Agent**: AI Agent Session X
- **Objectives**: [Main objectives for this session]

### Pre-Session Status
- **Backend**: [Status summary]
- **Frontend**: [Status summary]
- **Key Issues**: [Any blockers or issues]

### Work Completed
- ✅ [Achievement 1]
- ✅ [Achievement 2]
- ✅ [Achievement 3]

### Technical Details
[Detailed technical work performed]

### Quality Metrics Impact
- **Tests**: [Test status changes]
- **Coverage**: [Coverage changes]
- **Linting**: [Linting status]
- **Build**: [Build status]

### Files Modified/Created
[List of key files changed]

### Next Session Recommendations
[Suggested next steps]

### Session Quality Verification
```bash
[Commands run to verify quality]
```
```

### 2. PROGRESS.md ✅ (canonical — update this first)
**Update Required Sections:**
- [ ] [`#current-status`](PROGRESS.md#current-status): Last verified date, measured gate table, user-story table
- [ ] Do **not** add conflicting completion % elsewhere; historical notes go under Historical development log

### 3. QUALITY_METRICS.md ✅
**Update Required Sections:**
- [ ] Last Updated timestamp
- [ ] Measured gate results table (from fresh `npm run lint`, `test:run`, `cargo test`)
- [ ] Link to PROGRESS.md for user-story completion (no separate “overall quality %”)
- [ ] Lint policy: errors vs warnings (see `eslint.config.js`)

### 4. ISSUES.md ✅
**Update Required Actions:**
- [ ] Mark resolved issues as ✅ RESOLVED with resolution details
- [ ] Add new issues discovered during session
- [ ] Update issue priorities based on current project state
- [ ] Ensure all critical issues have proper status

### 5. AI_AGENT_INSTRUCTIONS.md ✅
**Update If Changed:**
- [ ] Project status overview
- [ ] Technical setup instructions
- [ ] Quality standards
- [ ] Testing framework status
- [ ] Known limitations or blockers

## 🎯 Session-Specific Updates

### Major Feature Implementation
- [ ] Update user story completion in PROGRESS.md
- [ ] Add new tests to quality metrics
- [ ] Document any new dependencies or setup requirements

### Bug Fixes
- [ ] Mark related issues as resolved in ISSUES.md
- [ ] Update quality metrics if tests were added/fixed
- [ ] Document fix details in SESSION_NOTES.md

### Infrastructure Changes
- [ ] Update AI_AGENT_INSTRUCTIONS.md with new setup requirements
- [ ] Update quality pipelines documentation
- [ ] Add new scripts to documentation

### Refactoring
- [ ] Update affected components in documentation
- [ ] Verify no breaking changes in quality metrics
- [ ] Update any architectural decisions

## 🔧 Post-Update Verification

### Run Documentation Verification Again
```bash
npm run doc:verify
```
**Expected Result:** ✅ ALL DOCUMENTATION VERIFICATION PASSED

### Run Full Quality Pipeline
```bash
npm run quality:full
```
**Expected Result:** All quality checks should pass

### Verify Consistency
- [ ] All timestamps should reflect current session date
- [ ] Test counts should be consistent across documents
- [ ] Issue statuses should match current project state
- [ ] Progress percentages should reflect actual completion

## 📊 Session Success Criteria

✅ **Documentation Complete** when:
- [ ] All 5 core documentation files updated
- [ ] `npm run doc:verify` passes without warnings
- [ ] All timestamps reflect current session
- [ ] Session notes capture all major work performed
- [ ] Progress reflects actual project advancement
- [ ] Quality metrics match current test/build status

## 🚀 Quality Integration

This checklist is integrated with the quality pipeline:
- Documentation verification runs as part of `npm run quality`
- Warnings are acceptable (exit code 2) for quality pipeline
- Failures (exit code 1) will block quality pipeline
- Use `npm run quality:full` for comprehensive checks including docs

## 💡 Best Practices

1. **Update as you go**: Don't wait until session end to update docs
2. **Be specific**: Include concrete numbers, file names, and outcomes
3. **Maintain consistency**: Use consistent terminology across documents
4. **Test your updates**: Always run doc verification after updates
5. **Link related work**: Reference user stories, issues, and technical decisions

---

**Remember:** Good documentation is as important as good code. This checklist ensures the next developer (AI or human) can understand exactly what was accomplished and what needs to happen next. 