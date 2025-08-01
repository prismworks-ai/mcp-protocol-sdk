# Public Repository Hygiene Guide

This document outlines how to maintain a clean separation between public and private/local development files.

## ✅ What SHOULD be in the public repository

### Core Project Files
- `src/` - All source code
- `tests/` - All test files
- `examples/` - Example implementations
- `benches/` - Benchmark code (not results)
- `docs/` - Documentation and guides
- `scripts/` - Development and CI scripts

### Configuration Files
- `Cargo.toml` - Project configuration
- `LICENSE` - License file
- `README.md` - Project overview
- `.gitignore` - Git ignore rules
- `clippy.toml`, `rustfmt.toml` - Code formatting config
- `deny.toml` - Dependency checking config
- `codecov.yml` - Coverage configuration

### CI/CD and GitHub
- `.github/` - GitHub configuration and workflows
- `Makefile` - Build automation

## ❌ What should NOT be in the public repository

### Generated Reports and Artifacts
- `tarpaulin-report.html` - Coverage reports
- `cobertura.xml` - Coverage XML
- `target/` - Build artifacts
- `Cargo.lock` - Dependency lock file (for libraries)

### Development Artifacts
- `dependency-analysis/` - Generated dependency reports
- `license-check-results/` - Generated license data
- `security-audit-results/` - Security audit results
- `benchmark-comparison/` - Benchmark result comparisons
- `.local-ci-status` - Local CI status files

### Personal/Local Files
- `.aicontext/` - AI development context
- `.local/` - Personal development files
- `scratch/`, `playground/` - Temporary development areas
- Personal notes, drafts, or temporary files

## 🔧 Local Development Structure

Use the `.local/` directory for development artifacts:

```
.local/
├── reports/           # Coverage reports, audit results
├── analysis/          # Code analysis results
├── benchmarks/        # Benchmark results and comparisons
└── notes/            # Personal development notes
```

## 📋 Maintenance Commands

### Generate reports locally
```bash
# Coverage report (goes to .local/reports/)
cargo tarpaulin --out html --output-dir .local/reports/

# Dependency analysis (goes to .local/analysis/)
cargo tree > .local/analysis/dependency-tree.txt

# Security audit (goes to .local/reports/)
cargo audit --json > .local/reports/audit-results.json
```

### Clean up accidentally committed files
```bash
# Remove from git history but keep locally
git rm --cached filename

# Remove from filesystem and git
git rm filename
```

### Check what's being tracked
```bash
# See all tracked files
git ls-files

# See ignored files
git status --ignored
```

## 🚨 Pre-commit Checklist

Before committing to public repository:

- [ ] No generated reports in commit
- [ ] No personal notes or temporary files
- [ ] No sensitive configuration data
- [ ] No large binary files or build artifacts
- [ ] AI context (`.aicontext/`) is ignored
- [ ] Coverage and audit results are in `.local/`

## 🔄 Regular Maintenance

1. **Weekly**: Review `.gitignore` for new patterns
2. **Before releases**: Clean up any accidentally tracked artifacts
3. **Monthly**: Review and clean `.local/` directory
4. **Before public pushes**: Run `git status --ignored` to verify

This ensures the public repository remains clean, professional, and contains only the essential files for users and contributors.
