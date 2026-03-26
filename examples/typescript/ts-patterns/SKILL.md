---
name: ts-patterns
description: >
  Core TypeScript patterns for the project including strict typing, error handling,
  async patterns, testing with vitest, and project structure conventions.
  Use when implementing TypeScript code or reviewing TS-specific patterns.
allowed-tools: Read, Glob, Grep
user-invocable: true
---

# TypeScript Patterns

Core TypeScript patterns and conventions for this project.

---

## 1. Strict Typing

```typescript
// Branded types for type safety
type UserId = string & { readonly __brand: "UserId" };
type OrderId = string & { readonly __brand: "OrderId" };

function createUserId(id: string): UserId {
  return id as UserId;
}

// Discriminated unions for state
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

// Exhaustive switch
function handleStatus(status: "active" | "inactive" | "pending"): string {
  switch (status) {
    case "active": return "Running";
    case "inactive": return "Stopped";
    case "pending": return "Waiting";
    default: {
      const _exhaustive: never = status;
      throw new Error(`Unhandled status: ${_exhaustive}`);
    }
  }
}
```

---

## 2. Error Handling

```typescript
class AppError extends Error {
  constructor(
    message: string,
    public readonly code: string,
    public readonly statusCode: number = 500,
  ) {
    super(message);
    this.name = "AppError";
  }
}

class NotFoundError extends AppError {
  constructor(entity: string, id: string) {
    super(`${entity} not found: ${id}`, "NOT_FOUND", 404);
  }
}

class ValidationError extends AppError {
  constructor(field: string, reason: string) {
    super(`Validation failed for ${field}: ${reason}`, "VALIDATION", 400);
  }
}
```

---

## 3. Async Patterns

```typescript
// Concurrent execution
const [users, orders] = await Promise.all([
  fetchUsers(),
  fetchOrders(),
]);

// Error handling with Result pattern
async function safeOperation<T>(
  fn: () => Promise<T>,
): Promise<Result<T>> {
  try {
    return { ok: true, value: await fn() };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

---

## 4. Testing with Vitest

```typescript
import { describe, it, expect, beforeEach } from "vitest";

describe("SearchService", () => {
  let service: SearchService;

  beforeEach(() => {
    service = new SearchService(createTestDb());
  });

  it("returns results for valid query", async () => {
    await service.insert(testItem);
    const results = await service.search("query", { k: 10 });
    expect(results).toHaveLength(1);
  });

  it("returns empty for no matches", async () => {
    const results = await service.search("nonexistent");
    expect(results).toEqual([]);
  });

  it.each([1, 10, 100])("respects k=%i limit", async (k) => {
    await seedDatabase(service, 200);
    const results = await service.search("query", { k });
    expect(results.length).toBeLessThanOrEqual(k);
  });
});
```

---

## 5. Project Structure

```
src/
├── index.ts          # Entry point / exports
├── config.ts         # Environment configuration
├── types/            # Shared type definitions
├── services/         # Business logic
├── routes/           # HTTP handlers (Express/Hono/etc.)
├── db/               # Database layer
│   ├── schema.ts     # Drizzle/Prisma schema
│   └── queries.ts    # Query functions
└── utils/            # Shared utilities

tests/
├── setup.ts          # Test configuration
├── unit/             # Unit tests
└── integration/      # Integration tests
```

---

## 6. Development Commands

```bash
# Install
pnpm install

# Dev
pnpm dev

# Test
pnpm test                  # All tests
pnpm test -- --run         # No watch mode
pnpm test -- --coverage    # With coverage

# Quality
pnpm lint                  # ESLint
pnpm format                # Prettier
pnpm typecheck             # tsc --noEmit
```
