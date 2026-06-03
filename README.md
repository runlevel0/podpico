# PodPico - AI-Assisted Desktop Podcast Manager

A modern, lightweight desktop application for managing podcast subscriptions and transferring episodes to USB devices, built with Tauri and Rust.

## 🤖 AI-Assisted Development System

This project is designed for **incremental development across multiple AI agent sessions**. Each session builds upon the previous work using a comprehensive tracking and handoff system.

### Quick Start for AI Agents

1. **Read the context files** in this order:
   - `ai_assisted_development/ProductOverview.md` - Product vision, user stories, and acceptance criteria
   - `ai_assisted_development/PROGRESS.md` - Current status and next priorities
   - `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md` - Complete development guidelines
   - `ai_assisted_development/SESSION_NOTES.md` - Historical session details
   - `ai_assisted_development/ISSUES.md` - Known blockers and technical debt
   - `ai_assisted_development/QUALITY_METRICS.md` - Quality status and targets

2. **Verify environment**:
   ```bash
   cd /home/patrick/Workspaces/podpico
   cargo check && npm run tauri dev --help
   ```

3. **Start development** following the session template in `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`

## 📋 Project Status

**Canonical source:** [`ai_assisted_development/PROGRESS.md`](ai_assisted_development/PROGRESS.md#current-status) (updated 2025-06-03). Do not duplicate percentages here without updating that file.

| Metric | Value |
|--------|-------|
| Phase | 1 (MVP core) |
| End-to-end complete user stories | **5/18 (28%)** — #1, #2, #5, #6, #7 |
| Partial (backend and/or UI; tests not fully green) | #3, #4, #8–#10 |
| Next priority | [Issue #2](https://github.com/runlevel0/podpico/issues/2) — failing Rust/frontend tests and ESLint errors |

**Latest measured gates:** `npm run build` pass; `npm run lint` fail (3 errors, 119 warnings); `npm run test:run` 90/3/6 (pass/fail/skip); `cargo clippy` pass; `cargo test --all` 109 pass / 3 fail. CI runs these on every PR (soft-fail on lint/tests until #2).

## 🏗️ Architecture

### Backend (Rust/Tauri)
- **Framework**: Tauri v2 with Rust backend
- **Database**: SQLite with SQLx
- **Error Handling**: Custom `PodPicoError` enum
- **Modules**: Complete structure with stub implementations

### Frontend (React/TypeScript)
- **Framework**: React 18 with TypeScript
- **Styling**: CSS (TBD: component library decision)
- **State Management**: TBD (React hooks vs Redux)

### Key Features (Implemented/Planned)

See user-story table in [`PROGRESS.md`](ai_assisted_development/PROGRESS.md#user-stories-18). Summary:

- ✅ RSS subscription, episode list, status tracking, combined inbox (#1, #2, #5–#7)
- 🟡 Download/remove podcast flows (#3–#4) — UI present; **tests failing** (issue #2)
- 🟡 USB detect/transfer/remove (#8–#10) — backend strong; frontend partial; integration incomplete
- ⏳ Settings, search, sync, and stories #11–#18 not started

## 🛠️ Development Environment

### Prerequisites
- Node.js 22.16.0+
- Rust **1.83.0** (pinned in `rust-toolchain.toml`; README previously cited 1.87.0+ — project builds on 1.83)
- Ubuntu/Debian system libraries (required for Tauri backend):
  ```bash
  sudo apt-get update
  sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
    libayatana-appindicator3-dev librsvg2-dev libsoup-3.0-dev libssl-dev pkg-config
  ```
- Then: `npm ci` (or `npm install`)

### Running the Application
```bash
# Development mode (hot reload)
npm run tauri dev

# Build for production
npm run tauri build

# Backend-only checks
cd src-tauri && cargo check
cd src-tauri && cargo test
```

### Testing
```bash
# Run all tests (interactive mode with watch)
npm test

# Run all tests once and exit (non-interactive, perfect for AI agents)
npm run test:run

# Run tests with coverage report
npm run test:coverage

# Run tests with verbose output (ideal for CI/debugging)
npm run test:ci

# Backend tests only
cd src-tauri && cargo test
```

**For AI Agents and Automation**: Always use `npm run test:run` to avoid interactive watch mode that can stall automated workflows.

## 📁 Project Structure

```
podpico/
├── src/                          # React frontend
│   ├── App.tsx                   # Main application component
│   └── ...                       # React components (TBD)
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs               # Tauri application entry point
│   │   ├── commands.rs          # Tauri command definitions
│   │   ├── database.rs          # SQLite database manager
│   │   ├── rss_manager.rs       # RSS feed handling
│   │   ├── usb_manager.rs       # USB device management
│   │   ├── file_manager.rs      # File operations
│   │   ├── episode_manager.rs   # Episode coordination
│   │   ├── config.rs            # Configuration management
│   │   └── error.rs             # Error type definitions
│   └── Cargo.toml               # Rust dependencies
├── ai_assisted_development/           # AI-assisted development files
│   ├── ImplementationPlan.md         # Comprehensive project plan
│   ├── PROGRESS.md                   # Session-by-session progress
│   ├── SESSION_NOTES.md              # Detailed session logs
│   ├── ISSUES.md                     # Known issues and blockers
│   ├── QUALITY_METRICS.md            # Quality tracking
│   ├── AI_AGENT_INSTRUCTIONS.md      # AI development guidelines
│   └── ProductOverview.md            # Product vision and requirements
├── package.json                  # Node.js dependencies
```

## 🎯 Development Phases

### Phase 1: MVP Core (Weeks 1-6)
- ✅ **Week 1-2**: Project setup and basic infrastructure (COMPLETED)
- ✅ **Week 3-4**: Podcast management - PARTIAL (5 user stories truly complete, 5 missing frontend)
- 🔄 **Week 5-6**: Frontend implementation debt resolution (IN PROGRESS - User Stories #3, #4, #8, #9, #10)

### Phase 2: Enhanced Features (Weeks 7-10)
- User interface polish and user experience improvements
- Search, filtering, and batch operations
- Advanced download and transfer management

### Phase 3: Quality & Distribution (Weeks 11-13)
- Comprehensive testing and optimization
- Cross-platform compatibility
- Application packaging and distribution

## 🔧 Technical Stack

### Core Dependencies
- **Tauri 2.0**: Desktop application framework
- **SQLx 0.8**: Async SQL toolkit with SQLite
- **Reqwest 0.12**: HTTP client for RSS feeds
- **RSS 2.0**: RSS feed parsing
- **Tokio 1.0**: Async runtime
- **React 18**: Frontend framework
- **TypeScript**: Type-safe JavaScript

### Development Tools
- **Cargo**: Rust package manager
- **NPM**: Node.js package manager
- **Rustfmt**: Code formatting
- **Clippy**: Rust linting (to be configured)

## 📊 Quality Standards

- **Compilation**: Must be warning-free
- **Error Handling**: All operations use `Result<T, PodPicoError>`
- **Testing**: Target 60% coverage for Phase 1
- **Documentation**: Comprehensive inline comments
- **Performance**: < 2s startup, < 200MB memory usage

## 🚀 Getting Started (For Human Developers)

1. **Clone and setup**:
   ```bash
   cd /home/patrick/Workspaces/podpico
   npm install
   ```

2. **Review the planning documents**:
   - Read `ai_assisted_development/ImplementationPlan.md` for complete architecture
   - Check `ai_assisted_development/PROGRESS.md` for current status
   - Review `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md` for development guidelines

3. **Start development**:
   ```bash
   npm run tauri dev
   ```

## 📝 Contributing

This project uses an AI-assisted development methodology. Each development session:

1. **Follows the session template** in `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`
2. **Updates progress tracking files** with current status
3. **Maintains high code quality** standards
4. **Documents issues and decisions** for future sessions
5. **Tests changes thoroughly** before handoff

## 📄 License

[Add license information]

## 🤝 Support

For development questions or issues:
1. Check `ai_assisted_development/ISSUES.md` for known problems
2. Review `ai_assisted_development/SESSION_NOTES.md` for historical context
3. Follow troubleshooting guides in `ai_assisted_development/AI_AGENT_INSTRUCTIONS.md`

---

## Historical README notes

Older README sections claimed 7/18 complete with #3–#4 “fully complete” while also listing #3–#4 as “frontend missing.” That contradiction is resolved in [`PROGRESS.md`](ai_assisted_development/PROGRESS.md#current-status) (canonical, measured 2025-06-03).

