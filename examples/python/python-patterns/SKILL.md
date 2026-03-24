---
name: python-patterns
description: >
  Core Python patterns for the project including type hints, error handling,
  async patterns with asyncio, testing with pytest, and project structure
  conventions. Use when implementing Python code or reviewing Python-specific patterns.
allowed-tools: Read, Glob, Grep
user-invocable: true
---

# Python Patterns

Core Python patterns and conventions for this project.

---

## 1. Type Hints

```python
from __future__ import annotations
from typing import Optional, Sequence
from dataclasses import dataclass, field

@dataclass(frozen=True)
class SearchResult:
    id: str
    score: float
    content: str
    metadata: dict[str, str] = field(default_factory=dict)

def search(query: str, k: int = 10) -> list[SearchResult]:
    ...
```

Use `from __future__ import annotations` for forward references. Prefer `list`, `dict`, `tuple` over `typing.List`, etc. (Python 3.9+).

---

## 2. Error Handling

```python
class AppError(Exception):
    """Base exception for the application."""

class NotFoundError(AppError):
    def __init__(self, entity: str, id: str):
        super().__init__(f"{entity} not found: {id}")
        self.entity = entity
        self.id = id

class ValidationError(AppError):
    def __init__(self, field: str, reason: str):
        super().__init__(f"Validation failed for {field}: {reason}")
```

### Best Practices

1. **Custom exceptions** over bare `Exception` or `ValueError`
2. **Catch specific exceptions**, not bare `except:`
3. **Use `raise ... from err`** for exception chaining
4. **Early return on validation** — validate inputs at function entry

---

## 3. Async Patterns

```python
import asyncio

async def fetch_all(urls: list[str]) -> list[Response]:
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_one(session, url) for url in urls]
        return await asyncio.gather(*tasks)

# Context managers for resource cleanup
async with db.transaction() as txn:
    await txn.insert(item)
    # Auto-commits on success, rolls back on exception
```

---

## 4. Testing with pytest

```python
import pytest

@pytest.fixture
def db(tmp_path):
    """Temporary database for testing."""
    return Database(tmp_path / "test.db")

class TestSearch:
    def test_search_returns_results(self, db):
        db.insert(item)
        results = db.search("query", k=10)
        assert len(results) > 0

    def test_search_empty_returns_empty(self, db):
        results = db.search("query", k=10)
        assert results == []

    @pytest.mark.parametrize("k", [1, 10, 100])
    def test_search_respects_k(self, db, k):
        seed_database(db, count=200)
        results = db.search("query", k=k)
        assert len(results) <= k
```

### Property-Based Testing with Hypothesis

```python
from hypothesis import given, strategies as st

@given(st.floats(min_value=0.0, max_value=1.0))
def test_importance_always_valid(importance):
    item = Item(importance=importance)
    assert 0.0 <= item.importance <= 1.0
```

---

## 5. Project Structure

```
src/
├── __init__.py
├── main.py          # Entry point
├── config.py        # Settings (pydantic-settings)
├── models/          # Data models
├── services/        # Business logic
├── api/             # HTTP handlers (FastAPI/Flask)
├── storage/         # Database layer
└── utils/           # Shared utilities

tests/
├── conftest.py      # Shared fixtures
├── unit/            # Unit tests
└── integration/     # Integration tests
```

---

## 6. Development Commands

```bash
# Install
uv sync                    # or pip install -e ".[dev]"

# Test
pytest                     # All tests
pytest -x                  # Stop on first failure
pytest --cov=src           # With coverage

# Quality
ruff check .               # Lint
ruff format .              # Format
mypy src/                  # Type check
```
