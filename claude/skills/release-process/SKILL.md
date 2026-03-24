---
name: release-process
description: >
  Standard operating procedure for releasing the project to its package registry.
  Use when preparing a release, bumping versions, running pre-release checks,
  publishing packages, or managing changelogs and git tags.
allowed-tools: Read, Glob, Grep, Bash
user-invocable: true
---

# Release Process

Standard operating procedure for releasing the project.

> **Related Skills**: [testing-protocol](../testing-protocol/SKILL.md) | [feature-development](../feature-development/SKILL.md)

---

## Versioning Strategy

### Semantic Versioning

Follow [SemVer](https://semver.org/): `MAJOR.MINOR.PATCH`

| Version Part | When to Bump | Example |
|--------------|--------------|---------|
| **MAJOR** | Breaking changes (post-1.0) | Removing public API |
| **MINOR** | New features; breaking changes (pre-1.0) | Adding new method |
| **PATCH** | Bug fixes, docs, internal changes | Fix crash on edge case |

### Pre-1.0 Expectations

While the project is `0.x.y`:
- **MINOR** bumps may include breaking changes
- **PATCH** bumps are always backwards-compatible
- API stability increases as you approach 1.0

### Version Decision Tree

```
Is this a breaking change?
├── YES
│   ├── Pre-1.0? → Bump MINOR (0.X.0)
│   └── Post-1.0? → Bump MAJOR (X.0.0)
└── NO
    ├── New feature? → Bump MINOR (0.X.0)
    └── Bug fix/docs? → Bump PATCH (0.0.X)
```

---

## What Constitutes a Breaking Change?

### Breaking (Require MINOR/MAJOR Bump)

- Remove public type, method, or endpoint
- Change return type or response schema
- Add required parameter
- Change error codes or variants
- Change stored data format (migration needed)

### Non-Breaking (PATCH is OK)

- Add new method, endpoint, or field
- Add optional parameter with default
- Add new error variant (if consumers use catch-all)
- Performance improvement
- Bug fix

---

## Deprecation Policy

1. **Mark deprecated** — Use language-appropriate deprecation annotation with migration path
2. **Document** — Note in CHANGELOG and docs
3. **Wait** — At least 2 MINOR versions
4. **Remove** — In next MAJOR (or MINOR if pre-1.0)

---

## Pre-Release Checklist

### Code Quality

- [ ] All tests passing (run the project's full test suite)
- [ ] Coverage maintained above threshold
- [ ] No linter warnings
- [ ] Code formatted

### Performance

- [ ] Benchmarks run (if applicable)
- [ ] No regression >10% from previous release

### Documentation

- [ ] Public API documented
- [ ] Examples compile/run
- [ ] CHANGELOG updated

### Security

- [ ] Dependencies audited (use language-appropriate audit tool)
- [ ] No hardcoded secrets
- [ ] Unsafe patterns justified and documented

---

## Release Procedure

### 1. Update Version

Edit the project's version file(s) (e.g., `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`).

### 2. Update CHANGELOG

Follow [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
# Changelog

## [Unreleased]

## [0.2.0] - YYYY-MM-DD

### Added
- New feature description (#issue)

### Changed
- Modified behavior description (breaking)

### Deprecated
- What's deprecated — use X instead

### Fixed
- Bug fix description (#issue)

### Performance
- Improvement description
```

### 3. Final Verification

Run the project's full quality suite:
- Full test suite
- Benchmarks (if applicable)
- Documentation builds
- Dry-run publish (if supported by registry)

### 4. Create Release Commit and Tag

```bash
git add <version-files> CHANGELOG.md
git commit -m "chore(release): v<VERSION>"
git tag -a v<VERSION> -m "Release v<VERSION>"
git push origin main --tags
```

### 5. Publish to Registry

Use the appropriate publish command for your ecosystem:
- **Rust**: `cargo publish`
- **npm**: `npm publish`
- **PyPI**: `python -m build && twine upload dist/*`
- **Go**: Tag push is sufficient (Go modules use git tags)

### 6. Create GitHub Release

```bash
gh release create v<VERSION> \
  --title "v<VERSION>" \
  --notes-file <(sed -n '/## \[<VERSION>\]/,/## \[/p' CHANGELOG.md | head -n -1)
```

---

## Post-Release Verification

- [ ] Registry listing is correct
- [ ] Test installation in a fresh project
- [ ] Update downstream dependencies if needed
- [ ] Announce in relevant channels (if significant)

---

## Rollback Procedure

### When to Rollback

| Severity | Condition | Action |
|----------|-----------|--------|
| **Critical** | Data loss, security vulnerability | Yank/unpublish immediately |
| **High** | Crash on common path | Yank + patch release |
| **Medium** | Bug in new feature | Patch release (no yank) |
| **Low** | Documentation error | Patch release |

### Emergency Patch Process

```bash
# 1. Create hotfix branch from tag
git checkout -b hotfix/<VERSION-PATCH> v<VERSION>

# 2. Fix the issue

# 3. Update version to patch

# 4. Update CHANGELOG with fix

# 5. Fast-track release (tests + publish)

# 6. Tag, merge back to main, push
git tag -a v<VERSION-PATCH> -m "Hotfix: <description>"
git checkout main
git merge hotfix/<VERSION-PATCH>
git push origin main --tags
```

---

## CHANGELOG Maintenance

Update CHANGELOG as you merge PRs, not just at release:

```markdown
## [Unreleased]

### Added
- New feature (#45)  <!-- Added when PR merged -->

### Fixed
- Bug fix (#46)
```

At release time:
1. Move `[Unreleased]` items to new version section
2. Add release date
3. Summarize for release notes

---

## Anti-Patterns

| Anti-Pattern | Why Bad | Fix |
|--------------|---------|-----|
| Releasing without tests/benchmarks | Hidden regressions | Always verify before release |
| Yanking for minor bugs | Breaks existing users | Patch release instead |
| Breaking changes in PATCH | Violates SemVer | Bump MINOR at minimum |
| Skipping CHANGELOG | Users don't know what changed | Update incrementally |
| No dry-run before publish | Catches packaging errors late | Always dry-run first |
| Releasing on Friday | Can't respond to issues quickly | Release early in week |
