---
name: rust-patterns
description: >
  Core Rust patterns for the project including error handling with thiserror,
  async patterns with tokio, API design (builder pattern, newtype), storage
  transaction patterns, and performance idioms. Use when implementing Rust code
  or reviewing Rust-specific patterns.
allowed-tools: Read, Glob, Grep
user-invocable: true
---

# Rust Patterns

Core Rust patterns and conventions for this project.

---

## 1. Error Handling with thiserror

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Storage error: {0}")]
    Storage(#[from] redb::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: &'static str, id: String },

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### Best Practices

1. **Never panic**: Always return Result in library code
2. **Use ?**: Propagate errors with ? operator
3. **Actionable messages**: Include context for debugging
4. **From implementations**: Use `#[from]` for automatic conversion

---

## 2. Async Patterns with Tokio

### Spawn Blocking for CPU-bound Work

```rust
let result = tokio::task::spawn_blocking(move || {
    // CPU-intensive or blocking I/O here
    db.expensive_operation()
}).await?;
```

### Concurrency with Arc

```rust
let db = Arc::new(Database::open(path)?);

let handles: Vec<_> = (0..10)
    .map(|i| {
        let db = db.clone();
        tokio::spawn(async move { db.read_operation(i).await })
    })
    .collect();

let results = futures::future::join_all(handles).await;
```

### Short-lived Write Transactions (SWMR)

```rust
// CORRECT: Short-lived write transaction
{
    let txn = db.begin_write()?;
    txn.insert(...)?;
    txn.commit()?;  // Releases write lock
}
// Other writes can proceed

// WRONG: Long-lived write transaction
let txn = db.begin_write()?;
// ... do other work ...  // Blocks ALL other writes!
txn.commit()?;
```

---

## 3. API Design

### Builder Pattern

```rust
pub struct SearchOptionsBuilder {
    k: usize,
    ef_search: Option<usize>,
    filter: Option<Filter>,
}

impl SearchOptionsBuilder {
    pub fn new(k: usize) -> Self {
        Self { k, ef_search: None, filter: None }
    }

    pub fn ef_search(mut self, ef: usize) -> Self {
        self.ef_search = Some(ef);
        self
    }

    pub fn filter(mut self, f: Filter) -> Self {
        self.filter = Some(f);
        self
    }

    pub fn build(self) -> SearchOptions {
        SearchOptions {
            k: self.k,
            ef_search: self.ef_search.unwrap_or(50),
            filter: self.filter,
        }
    }
}
```

### Newtype Pattern

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self { Self(uuid::Uuid::new_v4()) }
}
```

### Trait Design

```rust
#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn store(&self, item: NewItem) -> Result<ItemId>;
    async fn get(&self, id: ItemId) -> Result<Option<Item>>;
    async fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>>;
}
```

---

## 4. Documentation Standards

```rust
/// Records a new item in the store.
///
/// # Arguments
/// * `content` - Text content of the item
/// * `embedding` - Pre-computed embedding vector
///
/// # Returns
/// The ID of the newly created item.
///
/// # Errors
/// * `NotFound` - If the collection doesn't exist
/// * `DimensionMismatch` - If embedding dimension is wrong
///
/// # Example
/// ```rust
/// let id = store.record("learned something", &embedding).await?;
/// ```
pub async fn record(&self, content: &str, embedding: &[f32]) -> Result<ItemId> {
    // ...
}
```

---

## 5. Performance Idioms

- **Iterators over collecting**: `iter().filter().map()` instead of `collect()` then filter
- **Zero-copy where possible**: `&[u8]` over `Vec<u8>` for read-only access
- **Batch transactions**: Group multiple writes in a single transaction for one fsync
- **Avoid unnecessary allocations**: Reuse buffers, prefer `&str` over `String`
- **Use `#[inline]`** only on small functions called from hot paths (benchmark first)
