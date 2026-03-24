---
name: go-patterns
description: >
  Core Go patterns for the project including error handling, concurrency with
  goroutines/channels, testing with the standard library, and project structure
  conventions. Use when implementing Go code or reviewing Go-specific patterns.
allowed-tools: Read, Glob, Grep
user-invocable: true
---

# Go Patterns

Core Go patterns and conventions for this project.

---

## 1. Error Handling

```go
// Custom errors with sentinel values
var (
    ErrNotFound    = errors.New("not found")
    ErrValidation  = errors.New("validation failed")
)

// Structured errors
type AppError struct {
    Code    string
    Message string
    Err     error
}

func (e *AppError) Error() string { return e.Message }
func (e *AppError) Unwrap() error { return e.Err }

// Wrapping with context
func GetUser(id string) (*User, error) {
    user, err := db.Find(id)
    if err != nil {
        return nil, fmt.Errorf("GetUser(%s): %w", id, err)
    }
    return user, nil
}

// Checking wrapped errors
if errors.Is(err, ErrNotFound) { ... }
```

---

## 2. Concurrency

```go
// Worker pool pattern
func processItems(ctx context.Context, items []Item) error {
    g, ctx := errgroup.WithContext(ctx)
    sem := make(chan struct{}, 10) // Max 10 concurrent

    for _, item := range items {
        item := item // capture
        g.Go(func() error {
            sem <- struct{}{}
            defer func() { <-sem }()
            return process(ctx, item)
        })
    }

    return g.Wait()
}

// Context for cancellation
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
defer cancel()
```

---

## 3. Testing

```go
func TestSearch(t *testing.T) {
    db := setupTestDB(t)

    t.Run("returns results", func(t *testing.T) {
        db.Insert(testItem)
        results, err := db.Search("query", 10)
        require.NoError(t, err)
        assert.NotEmpty(t, results)
    })

    t.Run("empty query returns empty", func(t *testing.T) {
        results, err := db.Search("", 10)
        require.NoError(t, err)
        assert.Empty(t, results)
    })
}

// Table-driven tests
func TestValidation(t *testing.T) {
    tests := []struct {
        name    string
        input   string
        wantErr bool
    }{
        {"valid", "hello", false},
        {"empty", "", true},
        {"too long", strings.Repeat("a", 1001), true},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := Validate(tt.input)
            if tt.wantErr {
                assert.Error(t, err)
            } else {
                assert.NoError(t, err)
            }
        })
    }
}

// Fuzz testing (Go 1.18+)
func FuzzParse(f *testing.F) {
    f.Add("valid input")
    f.Fuzz(func(t *testing.T, input string) {
        _, _ = Parse(input) // Must not panic
    })
}
```

---

## 4. Project Structure

```
├── cmd/
│   └── server/
│       └── main.go       # Entry point
├── internal/             # Private packages
│   ├── config/           # Configuration
│   ├── domain/           # Business logic
│   ├── handler/          # HTTP handlers
│   └── storage/          # Database layer
├── pkg/                  # Public packages (if any)
├── go.mod
├── go.sum
└── Makefile
```

---

## 5. Development Commands

```bash
# Build
go build ./...

# Test
go test ./...                    # All tests
go test -v ./internal/domain/    # Verbose, specific package
go test -race ./...              # Race detector
go test -cover ./...             # Coverage
go test -fuzz=FuzzParse ./...    # Fuzz testing

# Quality
go vet ./...                     # Vet
golangci-lint run                # Lint
gofumpt -w .                     # Format
```
