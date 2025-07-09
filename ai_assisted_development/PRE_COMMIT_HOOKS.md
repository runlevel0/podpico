# PodPico Pre-Commit Hooks Documentation

**Last Updated**: 2025-01-15

## 🔍 Overview

PodPico uses Git pre-commit hooks to enforce quality standards automatically before code is committed. These hooks implement the zero-tolerance quality policy by running targeted checks on staged files.

## 🚀 Quick Start

### Installation

```bash
# From project root
./scripts/install-hooks.sh
```

This installs two hooks:
- **pre-commit**: Runs quality checks on staged files
- **commit-msg**: Validates commit message format

### Usage

The hooks run automatically when you commit:
```bash
git add .
git commit -m "feat: add new feature"
# Hooks will run quality checks and validate message
```

To bypass hooks (not recommended):
```bash
git commit --no-verify -m "emergency fix"
```

## 📋 Pre-Commit Hook Details

### What It Checks

The pre-commit hook performs intelligent, staged-file-aware quality checks:

#### Backend (Rust) Checks
- **Formatting**: Runs `rustfmt` on staged `.rs` files
- **Linting**: Runs `cargo clippy` with zero-warning policy
- **Tests**: Runs all backend tests

#### Frontend (TypeScript/React) Checks  
- **TypeScript**: Compilation check with strict mode
- **ESLint**: Zero-warning policy on staged files
- **Prettier**: Format validation on staged files
- **Tests**: Runs if < 10 files changed (performance optimization)

#### Documentation Checks
- **AI Docs**: Verifies consistency if AI development docs are modified
- **Warnings**: Documentation issues warn but don't block commits

### Performance Optimizations

- Only checks staged files (not entire codebase)
- Skips frontend tests for large changesets (>10 files)
- Provides timing information for each check
- Runs checks in parallel where possible

### Example Output

```
🔍 PodPico Pre-Commit Quality Checks
========================================
Running quality checks on staged files...

📦 Backend Quality Checks
-------------------------
⏳ Rust formatting... ✅ PASSED (1s)
⏳ Clippy (zero warnings)... ✅ PASSED (3s)
⏳ Rust tests... ✅ PASSED (5s)

🌐 Frontend Quality Checks
--------------------------
⏳ TypeScript compilation... ✅ PASSED (2s)
⏳ ESLint (zero warnings)... ✅ PASSED (1s)
⏳ Prettier formatting... ✅ PASSED (0s)
⏳ Frontend tests... ✅ PASSED (4s)

📊 Pre-Commit Summary
=====================
✅ All quality checks passed!
✅ Commit approved
   • Backend: All checks passed
   • Frontend: All checks passed
```

## 📝 Commit Message Hook Details

### Conventional Commits Format

The commit-msg hook enforces [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description

[optional body]

[optional footer(s)]
```

### Valid Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc)
- **refactor**: Code refactoring
- **test**: Test additions or changes
- **chore**: Build process or auxiliary tool changes
- **perf**: Performance improvements
- **ci**: CI/CD changes
- **build**: Build system changes
- **revert**: Revert a previous commit

### Examples

```bash
# Simple format
git commit -m "feat: add podcast download progress tracking"
git commit -m "fix: resolve USB device detection on Linux"
git commit -m "docs: update installation guide"

# With scope
git commit -m "fix(usb): handle device disconnection gracefully"
git commit -m "test(download): add edge case coverage"
git commit -m "refactor(rss): simplify feed parsing logic"

# AI Agent format
git commit -m "feat: Session 21 - User Story #3 - Download functionality

Implements User Story #3: Download episodes
- Added download progress tracking
- Implemented pause/resume functionality
- Added error recovery

Session: 2025-01-15 10:30:00
Co-authored-by: AI Agent Session 21 <2025-01-15 10:30:00>"
```

### Validation Rules

1. **Format**: Must follow conventional commit format
2. **Length**: First line ≤ 100 characters
3. **Blank Line**: Required after first line if body exists
4. **User Story**: Features should reference User Story # (warning only)
5. **AI Sessions**: Should include co-authored-by for AI commits

## 🛠️ Troubleshooting

### Common Issues

#### "Quality checks failed"
```bash
# See detailed error output
npm run quality      # Full quality check
npm run lint:fix     # Auto-fix linting issues
npm run format       # Auto-fix formatting
cd src-tauri && cargo fmt --all  # Fix Rust formatting
```

#### "Commit message validation failed"
- Check format matches: `type(scope): description`
- Ensure first line ≤ 100 characters
- Add blank line before body text
- Use valid type from the list above

#### "Hooks not running"
```bash
# Verify hooks are installed
ls -la .git/hooks/pre-commit
ls -la .git/hooks/commit-msg

# Reinstall if needed
./scripts/install-hooks.sh
```

### Bypassing Hooks

**⚠️ Use sparingly and only when necessary:**

```bash
# Skip all hooks
git commit --no-verify -m "emergency: fix critical production issue"

# Skip only pre-commit (keeps message validation)
git commit -n -m "feat: wip feature development"
```

## 🔧 Configuration

### Customizing Checks

Edit `.git/hooks/pre-commit` to modify checks:

```bash
# Example: Add custom check
if [ "$HAS_RUST_CHANGES" = true ]; then
    # Add security audit
    if ! run_check "Security audit" "cargo audit"; then
        FAILED_CHECKS+=("security")
    fi
fi
```

### Disabling Specific Checks

Comment out unwanted checks in the hook file:

```bash
# Example: Disable frontend tests in pre-commit
# if ! run_check "Frontend tests" "npm run test:run"; then
#     echo -e "${YELLOW}   Note: Tests skipped${NC}"
# fi
```

## 📊 Integration with CI/CD

The same quality standards enforced by pre-commit hooks are also run in CI/CD:

```yaml
# Example GitHub Actions integration
- name: Quality Checks
  run: |
    npm run quality:full
    cd src-tauri && make ci-check
```

## 🎯 Best Practices

1. **Run hooks locally**: Don't rely on CI/CD to catch issues
2. **Fix immediately**: Address quality issues before committing
3. **Small commits**: Easier to pass quality checks
4. **Descriptive messages**: Help future developers understand changes
5. **Reference stories**: Link features to User Stories

## 📚 Additional Resources

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Git Hooks Documentation](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- Project Quality Standards: `ai_assisted_development/QUALITY_METRICS.md`
- AI Agent Instructions: `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`

## 🔄 Maintenance

### Updating Hooks

After modifying hook scripts:
```bash
# Reinstall to apply changes
./scripts/install-hooks.sh
```

### Uninstalling Hooks

```bash
# Remove hooks
rm .git/hooks/pre-commit
rm .git/hooks/commit-msg
```

---

**Remember**: These hooks help maintain code quality and consistency across the project. They're designed to catch issues early and save time in the long run.