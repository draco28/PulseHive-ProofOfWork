---
name: testing-protocol
description: >
  Standard operating procedure for testing project components.
  Use when running tests, debugging test failures, setting up test environments,
  writing unit/integration/property-based/fuzz tests, or checking coverage.
  Covers test tier strategy, coverage requirements, async testing patterns,
  naming conventions, and anti-patterns.
allowed-tools: Read, Glob, Grep, Bash
user-invocable: true
---

# Testing Protocol

Standard operating procedure for testing project components.

> **Related Skills**: [release-process](../release-process/SKILL.md) | [feature-development](../feature-development/SKILL.md)

---

## Test Categories

### 1. Unit Tests

**Purpose**: Verify individual functions and types in isolation.

- Located alongside source code (inline modules or co-located test files)
- Fast, deterministic, no external dependencies
- Mock or stub external integrations

**What to unit test**: Pure logic, validation, serialization, data transformations, error conversions.

### 2. Integration Tests

**Purpose**: Verify multi-component workflows end-to-end.

- Located in a dedicated test directory (e.g., `tests/` in Rust, `tests/integration/` in Python)
- May use real databases, filesystems, or network calls (to local services)
- Slower but higher confidence

**What to integration test**: API workflows, data persistence roundtrips, multi-service interactions.

### 3. Property-Based Tests

**Purpose**: Verify invariants hold for ALL possible inputs, not just hand-picked examples.

**Frameworks by language**:
- Rust: `proptest`
- Python: `hypothesis`
- TypeScript: `fast-check`
- Go: `testing/quick` or `gopter`

**Key properties to test**:
- Roundtrip: `decode(encode(x)) == x`
- Bounds: output stays within expected range
- Idempotency: `f(f(x)) == f(x)` where applicable
- Monotonicity: adding data never reduces count

### 4. Fuzz Tests

**Purpose**: Find crashes, panics, and undefined behavior with malformed inputs.

- Particularly important for deserialization paths, parsers, and user-facing input handlers
- Run for extended periods before releases

**Frameworks by language**:
- Rust: `cargo-fuzz` with `arbitrary`
- Python: `atheris`
- Go: built-in `go test -fuzz`

---

## Test Naming Conventions

```
test_<component>_<scenario>_<expected_outcome>

Examples:
  test_user_create_with_valid_data_succeeds
  test_user_create_with_duplicate_email_fails
  test_search_with_empty_query_returns_empty
  test_delete_cascades_to_children
```

---

## Coverage Requirements

| Category | Target | Rationale |
|----------|--------|-----------|
| Overall | >80% | Industry standard for libraries and services |
| Critical paths | 100% | Core business logic, data integrity operations |
| Public API | 100% | Every public function has at least one test |
| Error paths | >90% | Error handling must be verified |

Use the project's coverage tool as specified in CLAUDE.md.

---

## Test Execution Procedure

### Before Every PR

Run these commands from CLAUDE.md's Development Commands section:

1. Run all tests
2. Run with release/production optimizations (catches different bugs)
3. Check coverage against threshold
4. Run linter
5. Check formatting

### Before Release

1. Extended property tests (higher iteration count)
2. Extended fuzzing (longer duration)
3. Cross-platform verification (if applicable)

### Investigating Failures

1. **Reproduce locally**: Run the specific failing test with verbose output
2. **Check for flakiness**: Run 10x with single-threaded execution
3. **Enable debug logging**: Set appropriate log level env var
4. **Check minimization**: For property tests, review minimized failing case

---

## Test Fixtures & Data

### Temporary Resources

Always use temporary directories/databases that are cleaned up on teardown:
- Prevents test pollution
- Enables parallel test execution
- No manual cleanup needed

### Test Data Generators

Create helper functions for generating valid test data:
- Random but valid inputs (respect invariants)
- Seeded randomness for reproducibility when needed
- Edge case generators (empty, max-size, unicode, etc.)

### Seeding Test Data

For integration tests that need pre-populated data:
- Create bulk insert helpers
- Use consistent seed data across related tests
- Document assumptions about seed state

---

## Async Testing Patterns

### Basic Async Test
Use your language's async test runner (e.g., `#[tokio::test]`, `pytest.mark.asyncio`, etc.)

### Async Test with Timeout
Wrap long-running operations with a timeout to prevent hanging tests.

### Concurrent Test Scenarios
Test concurrent access patterns:
- Multiple readers
- Reader/writer contention
- Verify all operations complete without deadlocks

---

## Anti-Patterns

| Anti-Pattern | Why Bad | Fix |
|--------------|---------|-----|
| Ignored/skipped tests without reason | Hidden tech debt | Add issue tracker reference |
| Bare assertions without context | Unclear failure messages | Use descriptive assertion messages |
| Shared mutable state between tests | Flaky, order-dependent | Fresh fixture per test |
| Sleep for timing | Slow and flaky | Use synchronization primitives |
| Hardcoded paths | Breaks on CI/other machines | Use temp directories |
| Testing implementation, not behavior | Brittle, breaks on refactor | Test public API contracts |
| Mocking everything | False confidence | Prefer integration tests for critical paths |

---

## Project-Specific Patterns

Consult your project's patterns skill (e.g., `rust-patterns`, `python-patterns`) for:
- Language-specific test frameworks and commands
- Domain-specific test fixtures and generators
- Project-specific invariants and properties to test
