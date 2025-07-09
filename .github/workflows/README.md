# PodPico CI/CD Workflows

This directory contains GitHub Actions workflows that implement comprehensive CI/CD pipelines for the PodPico project, enforcing the zero-tolerance quality policy defined in the AI Agent Instructions.

## 🎯 Overview

Our CI/CD system enforces strict quality standards:
- **Zero warnings** policy (ESLint, Clippy)
- **80% minimum** code coverage
- **All tests must pass** before merging
- **Security scanning** for vulnerabilities
- **Cross-platform** build verification

## 📋 Workflows

### 1. Continuous Integration (`ci.yml`)
**Trigger**: Push to main/develop, Pull requests  
**Purpose**: Comprehensive quality checks on every code change

**Jobs**:
- `frontend-quality`: TypeScript, ESLint, Prettier, tests, coverage
- `backend-quality`: Rust formatting, Clippy, tests, coverage (80% minimum)
- `integration-tests`: Full application build and E2E tests (when implemented)
- `documentation`: Verify documentation consistency
- `security`: Run cargo-audit and npm audit
- `build-verification`: Cross-platform build tests (Linux, Windows, macOS)
- `quality-gate`: Final summary and merge decision

### 2. Pull Request Quality (`pr-quality.yml`)
**Trigger**: Pull request events  
**Purpose**: Detailed PR feedback with coverage reports and quality metrics

**Features**:
- Automated PR comments with:
  - Coverage reports (frontend & backend)
  - Linting issues with specific line numbers
  - Test results summary
  - Quality gate status
- Quick formatting checks
- Merge requirement checklist

### 3. Nightly Quality Check (`nightly.yml`)
**Trigger**: Daily at 2 AM UTC, Manual dispatch  
**Purpose**: Deep quality analysis and maintenance

**Jobs**:
- `quality-analysis`: Full test suite with detailed coverage
- `security-audit`: Comprehensive vulnerability scanning
- `dependency-updates`: Check for outdated dependencies
- `performance-check`: Build time and resource usage metrics
- `create-report`: Generate GitHub issue with findings

**Outputs**:
- Nightly quality report issue
- Dependency update PR (if needed)
- Performance benchmarks
- Security audit reports

### 4. Release (`release.yml`)
**Trigger**: Git tags (v*), Manual dispatch  
**Purpose**: Automated release builds with quality gates

**Process**:
1. Enforce all quality gates (tests, coverage, linting)
2. Build release artifacts for all platforms
3. Generate release notes from commits
4. Create draft GitHub release
5. Upload all artifacts

## 🔧 Configuration

### Required Secrets
- `GITHUB_TOKEN`: Automatically provided by GitHub
- `TAURI_PRIVATE_KEY`: For signing Tauri releases
- `TAURI_KEY_PASSWORD`: Password for Tauri signing key

### Environment Variables
- `CARGO_TERM_COLOR`: Always enabled for colored output
- `RUST_BACKTRACE`: Set to 1 for debugging

## 📊 Quality Gates

All workflows enforce these quality standards:

| Check | Requirement | Enforcement |
|-------|------------|-------------|
| ESLint | Zero warnings | Fails CI |
| Clippy | Zero warnings | Fails CI |
| Tests | 100% pass rate | Fails CI |
| Coverage | ≥80% (backend) | Fails CI |
| Formatting | Consistent | Fails CI |
| TypeScript | No type errors | Fails CI |
| Security | No high vulnerabilities | Warning only |

## 🚀 Usage

### Running Workflows Manually

Most workflows support manual dispatch:

```bash
# Trigger nightly check manually
gh workflow run nightly.yml

# Create a release
gh workflow run release.yml -f version=v0.1.0
```

### PR Workflow
1. Create PR → `pr-quality.yml` runs automatically
2. Review automated comments for issues
3. Fix any quality gate failures
4. All checks must pass before merge

### Release Process
1. Tag commit: `git tag v0.1.0 && git push --tags`
2. `release.yml` runs automatically
3. Review draft release on GitHub
4. Publish when ready

## 🔍 Monitoring

### Workflow Status
- Check Actions tab for workflow runs
- Failed workflows trigger GitHub notifications
- Nightly reports create tracking issues

### Coverage Trends
- Coverage reports uploaded as artifacts
- Can integrate with Codecov for trends
- Nightly workflow tracks coverage over time

### Security Monitoring
- Daily vulnerability scans
- Dependency update PRs for security fixes
- Security reports stored for 90 days

## 📈 Best Practices

1. **Never skip quality gates** - They exist for a reason
2. **Fix warnings immediately** - Zero tolerance policy
3. **Review nightly reports** - Catch issues early
4. **Keep dependencies updated** - Security and performance
5. **Monitor coverage trends** - Don't let it slip below 80%

## 🆘 Troubleshooting

### Common Issues

**ESLint/Clippy warnings**:
```bash
# Fix automatically
npm run lint:fix
cd src-tauri && cargo clippy --fix
```

**Coverage below threshold**:
- Write more tests for uncovered code
- Check coverage reports in artifacts

**Build failures**:
- Check system dependencies
- Review error logs in workflow run

**Slow workflows**:
- Caching reduces build times
- Parallel jobs improve speed

## 📚 Related Documentation

- [AI Agent Instructions](../../ai_assisted_development/AI_AGENT_INSTRUCTIONS.md)
- [Quality Metrics](../../ai_assisted_development/QUALITY_METRICS.md)
- [Testing Conventions](../../ai_assisted_development/TEST_NAMING_CONVENTIONS.md)