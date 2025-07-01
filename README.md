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

- **Phase**: 1 (MVP Core)
- **Week**: 6 (Major Frontend Implementation Progress)
- **Overall Progress**: 39% Complete (7/18 user stories truly functional) 🟢 **UP FROM 28% - MAJOR PROGRESS**
- **Frontend Debt**: Reduced from 5 to 3 user stories (40% reduction achieved!)
- **Next Priority**: Continue Frontend Implementation for Backend-Ready User Stories (#8, #9, #10)
- **Success**: User Stories #3 (Download) and #4 (Remove Podcasts) frontends complete

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
- ✅ RSS feed podcast subscription management (Full frontend ✅)
- ✅ Episode download with progress tracking (Full frontend ✅)
- ✅ Podcast removal with safety confirmations (Full frontend ✅)
- 🔄 USB device detection and file transfer (detection ✅, transfer ✅, frontend pending)
- ✅ Episode status tracking (New/Unlistened/Listened)
- ⏳ Configurable settings and preferences

## 🛠️ Development Environment

### Prerequisites
- Node.js 22.16.0+
- Rust 1.87.0+
- System dependencies (installed):
  - webkit2gtk-4.0-dev
  - build-essential
  - libssl-dev

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

## 🚀 **MAJOR PROGRESS UPDATE** (Date: 2025-06-29)

**Frontend Implementation Breakthrough**: User Stories #3 and #4 Complete!  
**Current Status**: 7 user stories have complete frontend implementations (up from 5)

### ✅ **TRULY COMPLETE User Stories (7/18)**:
- User Story #1: Add podcast subscription ✅
- User Story #2: View episodes ✅  
- User Story #3: Download episodes with progress tracking ✅ **NEW!**
- User Story #4: Remove podcasts with confirmations ✅ **NEW!**
- User Story #5: Mark episodes as listened ✅
- User Story #6: Episode status indicators ✅
- User Story #7: Combined inbox ✅

### 🟡 **BACKEND COMPLETE, FRONTEND MISSING (5/18)**:
- User Story #3: Download episodes ❌ Frontend missing
- User Story #4: Remove podcasts ❌ Frontend missing
- User Story #8: USB device capacity ❌ Frontend missing
- User Story #9: Transfer to USB ❌ Frontend missing
- User Story #10: Remove from USB ❌ Frontend missing

### Architecture Section (if applicable)
Update key features list when new major features are implemented:

### Key Features (Implemented/Planned)
- ✅ RSS feed podcast subscription management
- ✅ Episode list display and status management  
- ✅ Episode status tracking (New/Unlistened/Listened)
- ✅ Combined inbox for new episodes across podcasts
- ✅ Search functionality within podcasts
- 🟡 Episode download (Backend ✅, Frontend ❌)
- 🟡 USB device detection (Backend ✅, Frontend ❌)
- 🟡 Episode transfer to USB (Backend ✅, Frontend ❌)
- 🟡 Podcast removal (Backend ✅, Frontend ❌)
- ⏳ Configurable settings and preferences

**Current Status**: Frontend implementation debt resolution required for 50% of implemented features

