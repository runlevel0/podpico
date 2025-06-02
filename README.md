# PodPico - AI-Assisted Desktop Podcast Manager

A modern, lightweight desktop application for managing podcast subscriptions and transferring episodes to USB devices, built with Tauri and Rust.

## 🤖 AI-Assisted Development System

This project is designed for **incremental development across multiple AI agent sessions**. Each session builds upon the previous work using a comprehensive tracking and handoff system.

### Quick Start for AI Agents

1. **Read the context files** in this order:
   - `ProductOverview.md` - Product vision, user stories, and acceptance criteria
   - `PROGRESS.md` - Current status and next priorities
   - `AI_AGENT_INSTRUCTIONS.md` - Complete development guidelines
   - `SESSION_NOTES.md` - Historical session details
   - `ISSUES.md` - Known blockers and technical debt
   - `QUALITY_METRICS.md` - Quality status and targets

2. **Verify environment**:
   ```bash
   cd /home/patrick/Workspaces/podpico
   cargo check && npm run tauri dev --help
   ```

3. **Start development** following the session template in `AI_AGENT_INSTRUCTIONS.md`

## 📋 Project Status

- **Phase**: 1 (MVP Core)
- **Week**: 1-2 (Project Setup & Basic Infrastructure) ✅ COMPLETED
- **Overall Progress**: 15% (Initial setup completed)
- **Next Priority**: Database schema implementation

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

### Key Features (Planned)
- 📻 RSS feed podcast subscription management
- 📥 Episode download and local storage
- 🔌 USB device detection and file transfer
- 📊 Episode status tracking (New/Unlistened/Listened)
- 🎛️ Configurable settings and preferences

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
├── ImplementationPlan.md         # Comprehensive project plan
├── PROGRESS.md                   # Session-by-session progress
├── SESSION_NOTES.md              # Detailed session logs
├── ISSUES.md                     # Known issues and blockers
├── QUALITY_METRICS.md            # Quality tracking
├── AI_AGENT_INSTRUCTIONS.md      # AI development guidelines
└── package.json                  # Node.js dependencies
```

## 🎯 Development Phases

### Phase 1: MVP Core (Weeks 1-6)
- ✅ **Week 1-2**: Project setup and basic infrastructure (COMPLETED)
- 🔄 **Week 3-4**: Podcast management (RSS parsing, CRUD operations)
- ⏳ **Week 5-6**: File operations and USB integration

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
   - Read `ImplementationPlan.md` for complete architecture
   - Check `PROGRESS.md` for current status
   - Review `AI_AGENT_INSTRUCTIONS.md` for development guidelines

3. **Start development**:
   ```bash
   npm run tauri dev
   ```

## 📝 Contributing

This project uses an AI-assisted development methodology. Each development session:

1. **Follows the session template** in `AI_AGENT_INSTRUCTIONS.md`
2. **Updates progress tracking files** with current status
3. **Maintains high code quality** standards
4. **Documents issues and decisions** for future sessions
5. **Tests changes thoroughly** before handoff

## 📄 License

[Add license information]

## 🤝 Support

For development questions or issues:
1. Check `ISSUES.md` for known problems
2. Review `SESSION_NOTES.md` for historical context
3. Follow troubleshooting guides in `AI_AGENT_INSTRUCTIONS.md`

---

**Current Status**: Ready for next development session (Database schema implementation)
