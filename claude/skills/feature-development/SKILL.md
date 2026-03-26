---
name: feature-development
description: >
  Step-by-step procedure for developing new features, from design through delivery.
  Use when starting a new feature, creating a branch, implementing with project
  patterns, writing tests, validating performance, or preparing a PR.
allowed-tools: Read, Glob, Grep, Bash
user-invocable: true
---

# Feature Development SOP

Step-by-step procedure for developing new features, from design through delivery.

> **Related Skills**: [testing-protocol](../testing-protocol/SKILL.md) | [release-process](../release-process/SKILL.md)

---

## Feature Development Phases

```
1. Design → 2. Implement → 3. Test → 4. Validate → 5. Review → 6. Merge
```

---

## Phase 1: Design & Architecture Review

### When to Do Architecture Review

| Feature Type | Review Level |
|--------------|--------------|
| New public API or endpoint | **Full review** |
| New data model or schema change | **Full review** |
| Core algorithm change | **Full review** |
| Internal refactoring | Light review |
| Bug fix | Skip (unless architectural) |
| Documentation | Skip |

### Design Questions

- [ ] What's the API surface? (public types, methods, endpoints)
- [ ] What data is stored/modified? (schema, serialization format)
- [ ] What are the failure modes? (errors that can occur)
- [ ] What are the performance implications?
- [ ] Does this affect backwards compatibility?

### Design Document Template

For complex features, create a brief design doc:

```markdown
# Feature: <Name>

## Problem
<What problem does this solve?>

## Proposed Solution
<High-level approach>

## API Design
<New public types/methods/endpoints with signatures>

## Data Model Changes
<New tables, schema changes, migration steps>

## Performance Impact
<Expected impact on benchmarks or latency>

## Alternatives Considered
<Why not other approaches?>
```

Consult your project's patterns skill for domain-specific design conventions.

---

## Phase 2: Implementation

### Create Branch

```bash
git checkout main
git pull origin main
git checkout -b feat/<feature-name>
```

**If ProjectPulse is configured:**
```bash
git checkout -b feat/ticket-<N>-<slug>
```
And start an agent session:
```
projectpulse_agent_session_start({
  projectId: <ID>,
  name: "Feature: <title>",
  activeTicketNumbers: [<N>]
})
```

### Implementation Guidelines

Consult your project's patterns skill for language-specific conventions including:
- Concurrency patterns
- Error handling idioms
- Data access patterns
- File organization conventions

### Documentation Standards

All public API should be documented with:
- Description of what the function/method/endpoint does
- Parameter descriptions
- Return value description
- Error conditions
- Usage example

---

## Phase 3: Testing

### Test Requirements by Feature Type

| Feature Type | Unit Tests | Integration Tests | Property Tests | Benchmarks |
|--------------|------------|-------------------|----------------|------------|
| New public API | Required | Required | Recommended | If perf-sensitive |
| Internal refactor | Required | If behavior changes | Optional | If perf-sensitive |
| Bug fix | Required (regression) | If multi-component | Optional | Optional |

### Integration Checkpoint

Before proceeding to Phase 4, run the project's quality checks (from CLAUDE.md Development Commands):
- All tests pass
- Coverage maintained
- No linter warnings
- Code formatted

> **Reference**: See [testing-protocol](../testing-protocol/SKILL.md) for detailed testing patterns.

---

## Phase 4: Performance Validation

### When Required

- Touches a critical/hot path
- Introduces new data structure or algorithm
- Modifies query/search logic
- Changes serialization format

### Procedure

```bash
# 1. Save baseline on main
git stash && git checkout main
# Run benchmarks and save baseline
git checkout - && git stash pop

# 2. Run benchmarks on feature branch
# Compare against baseline

# 3. Verify no regression >10%
```

---

## Phase 5: Code Review

### Self-Review Checklist

Before requesting review:

- [ ] Code compiles/builds with no warnings
- [ ] All tests pass
- [ ] Coverage maintained above threshold
- [ ] Documentation complete for public API
- [ ] No unhandled errors or panics in library/service code
- [ ] Error messages are helpful and actionable
- [ ] Performance validated (if applicable)

### PR Description Template

```markdown
## Summary
<1-2 sentences describing the change>

## Changes
- <Bullet points of what changed>

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Performance
- [ ] Benchmarks run (no regression >10%)
- [ ] N/A (not performance-sensitive)

## Breaking Changes
- [ ] None
- [ ] <Description and migration path>
```

---

## Phase 6: Merge & Release Impact

### Version Implications

| Change Type | Version Bump |
|-------------|--------------|
| New feature (additive) | Minor |
| Bug fix | Patch |
| Breaking change (pre-1.0) | Minor |
| Breaking change (post-1.0) | Major |

### Before Merging

- [ ] Does downstream code need updates?
- [ ] Are there migration steps for existing data?
- [ ] Should this be mentioned in release notes?

**If ProjectPulse is configured:**
```
projectpulse_agent_session_end({ sessionId: "..." })
# Ticket auto-moves to "in-review"
```

> **Reference**: See [release-process](../release-process/SKILL.md) for release workflow.

---

## Scope Management

### Breaking Features Into Tasks

For features taking >1 day:

```
Epic: Add filtered search
├── Task 1: Add Filter type and validation
├── Task 2: Implement filter evaluation
├── Task 3: Integrate with search endpoint
├── Task 4: Add benchmarks
└── Task 5: Documentation and examples
```

### Definition of Done

A feature is "done" when:

1. **Code complete** — Implementation matches design
2. **Tests passing** — All automated tests green
3. **Documented** — Public API documented with examples
4. **Reviewed** — At least one approval
5. **Merged** — In main branch
6. **Validated** — Performance verified (if applicable)

---

## Anti-Patterns

| Anti-Pattern | Why Bad | Fix |
|--------------|---------|-----|
| Skipping design for "small" features | Small features grow | At least mental review |
| Testing only happy path | Bugs in error handling | Test error cases explicitly |
| Unhandled errors in library/service code | Crashes in production | Return proper error types |
| Long-lived transactions or locks | Blocks other operations | Keep critical sections short |
| Changing public API without docs | Users can't migrate | Document all changes |
| Merging without benchmarks (hot path) | Hidden performance regression | Benchmark before merge |
