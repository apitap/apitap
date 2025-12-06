# Rust Async Streams Guide: BoxStream, Pin, and Related Concepts

A comprehensive guide to understanding async streams in Rust, covering BoxStream, Pin, lifetimes, and practical usage patterns.

## Table of Contents
1. [BoxStream Fundamentals](#boxstream-fundamentals)
2. [Pin and Pinning](#pin-and-pinning)
3. [Lifetimes in Streams](#lifetimes-in-streams)
4. [Type Conversions](#type-conversions)
5. [Practical Patterns](#practical-patterns)
6. [Common Pitfalls](#common-pitfalls)

---

## BoxStream Fundamentals

### What is BoxStream?

`BoxStream<'a, T>` is a type alias from the `futures` crate:

```rust
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;
```

**Breaking it down:**
- `Box<...>` - Heap-allocated (dynamic dispatch)
- `dyn Stream<Item = T>` - Trait object for any stream type
- `+ Send` - Can be sent across threads
- `Pin<...>` - Memory location is fixed (won't move)
- `'a` - Lifetime parameter (how long data lives)

### BoxStream vs Manual Type

**Without BoxStream (verbose):**
```rust
Pin<Box<dyn Stream<Item = Result<Value>> + Send + 'static>>
```

**With BoxStream (clean):**
```rust
BoxStream<'static, Result<Value>>
```

**They're identical!** BoxStream is just a convenient shorthand.

### Real-World Example

```rust
// Function returning a stream
async fn fetch_data() -> BoxStream<'static, Result<Value>> {
    let items = vec![json!({"id": 1}), json!({"id": 2})];
    stream::iter(items.into_iter().map(Ok)).boxed()
}
```

---

## Pin and Pinning

### Why Pin Exists

**Problem:** Async streams may contain self-referential data (pointers to their own fields). If the stream moves in memory, these internal pointers become invalid.

**Solution:** `Pin<T>` guarantees the value won't move in memory.

### Three Ways to Pin

#### 1. Stack Pinning (Recommended)
```rust
let mut stream = some_stream;
let mut pinned = std::pin::pin!(stream);  // Pin on stack (no allocation)

while let Some(item) = pinned.next().await {
    // use item
}
```

**Pros:** No heap allocation, fast  
**Type:** `Pin<&mut T>`

#### 2. Heap Pinning
```rust
let pinned = Box::pin(stream);  // Pin on heap (allocates)

while let Some(item) = pinned.next().await {
    // use item
}
```

**Pros:** Owned, can return from functions  
**Type:** `Pin<Box<T>>`

#### 3. Legacy futures::pin_mut!
```rust
futures::pin_mut!(stream);  // Older style

while let Some(item) = stream.next().await {
    // use item
}
```

### When Pinning is Required

```rust
// ❌ This won't compile:
let mut stream = some_stream;
while let Some(item) = stream.next().await {
    // ERROR: `next()` requires Pin<&mut Self>
}

// ✅ This works:
let mut pinned = std::pin::pin!(stream);
while let Some(item) = pinned.next().await {
    // OK
}
```

---

## Lifetimes in Streams

### 'static Lifetime

**Meaning:** The stream owns all its data, no borrowed references.

```rust
BoxStream<'static, Result<Value>>
```

**When to use:**
- Stream owns its data
- Returning streams from functions
- Sending streams across tasks
- Maximum flexibility

**Example:**
```rust
fn create_stream() -> BoxStream<'static, Result<Value>> {
    let data = vec![json!({"id": 1})];  // Owned data
    stream::iter(data.into_iter().map(Ok)).boxed()
}
```

### 'a Lifetime (Borrowed)

**Meaning:** The stream may borrow data that lives for `'a`.

```rust
BoxStream<'a, Result<&'a Value>>
```

**When to use:**
- Stream references external data
- Avoiding clones for efficiency
- Data has a known, limited scope

**Example:**
```rust
fn create_borrowed_stream<'a>(
    data: &'a Vec<Value>
) -> BoxStream<'a, Result<&'a Value>> {
    stream::iter(data.iter().map(Ok)).boxed()
}
```

### Comparison

| Feature | 'static | 'a |
|---------|---------|-----|
| Flexibility | Maximum | Limited |
| Memory | May clone data | References data |
| Usage | Return from functions | Local/scoped |
| Thread safety | Yes | Depends |

**Rule of thumb:** Use `'static` unless you specifically need to borrow data.

---

## Type Conversions

### Box::pin vs .boxed()

Both do the same thing - convert a stream to a boxed, pinned trait object:

```rust
// Using Box::pin
let stream = Box::pin(my_stream);

// Using .boxed() (convenience method)
let stream = my_stream.boxed();
```

**They're equivalent!** `.boxed()` calls `Box::pin(self)` internally.

### When Box::pin is Required

**Scenario:** Converting concrete type to trait object

```rust
// counted_stream has concrete type: Map<BoxStream, Closure>
let counted_stream = s.map(|result| { ... });

// Function expects trait object: Pin<Box<dyn Stream>>
writer.write_page_stream(
    Box::pin(counted_stream),  // Must convert
    write_mode
).await?;
```

### Type Alias Simplification

**Before:**
```rust
pub type StreamFactory = 
    Arc<dyn Fn() -> Pin<Box<dyn Stream<Item = Result<Value>> + Send>> + Send + Sync>;
```

**After (with BoxStream):**
```rust
pub type StreamFactory = 
    Arc<dyn Fn() -> BoxStream<'static, Result<Value>> + Send + Sync>;
```

---

## Practical Patterns

### Pattern 1: Stream Factory

**Why:** Create fresh streams for each use (e.g., DataFusion partitions)

```rust
pub type JsonStreamFactory = 
    Arc<dyn Fn() -> BoxStream<'static, Result<Value>> + Send + Sync>;

// Usage
let factory: JsonStreamFactory = Arc::new(|| {
    let data = vec![...];
    stream::iter(data.into_iter().map(Ok)).boxed()
});

// Each call creates a new independent stream
let stream1 = factory();
let stream2 = factory();
```

### Pattern 2: Stream Transformation

```rust
async fn transform_stream(
    input: BoxStream<'static, Result<Value>>
) -> BoxStream<'static, Result<Value>> {
    input
        .map(|result| {
            result.map(|v| {
                // Transform value
                json!({"transformed": v})
            })
        })
        .boxed()  // Convert back to BoxStream
}
```

### Pattern 3: Counting Stream Items

```rust
let count = Arc::new(AtomicUsize::new(0));
let count_clone = count.clone();

let counted_stream = stream.map(move |result| {
    if result.is_ok() {
        count_clone.fetch_add(1, Ordering::Relaxed);
    }
    result
});

// Process stream...
let total = count.load(Ordering::Relaxed);
```

### Pattern 4: RecordBatchStream Adapter

```rust
// ❌ Wrong - boxes as Stream
let adapter = RecordBatchStreamAdapter::new(schema, stream.boxed());
Ok(adapter.boxed())  // ERROR: Wrong trait!

// ✅ Correct - boxes as RecordBatchStream
let adapter = RecordBatchStreamAdapter::new(schema, stream.boxed());
Ok(Box::pin(adapter))  // OK: Correct trait
```

---

## Common Pitfalls

### Pitfall 1: Missing Lifetime Parameter

```rust
// ❌ Won't compile
BoxStream<Result<Value>>

// ✅ Correct
BoxStream<'static, Result<Value>>
```

**Fix:** Always specify the lifetime parameter.

### Pitfall 2: Wrong Trait Bound

```rust
// ❌ Wrong trait
Pin<Box<dyn Stream<Item = T> + Send>>  // Just Stream

// ✅ Correct trait for DataFusion
Pin<Box<dyn RecordBatchStream + Send>>  // RecordBatchStream extends Stream
```

**Fix:** Use `Box::pin(adapter)` for RecordBatchStream, not `.boxed()`.

### Pitfall 3: Forgetting to Pin

```rust
// ❌ Won't compile
let mut stream = create_stream();
while let Some(item) = stream.next().await {
    // ERROR: needs Pin<&mut Self>
}

// ✅ Correct
let mut pinned = std::pin::pin!(stream);
while let Some(item) = pinned.next().await {
    // OK
}
```

**Fix:** Use `std::pin::pin!()` macro.

### Pitfall 4: Unnecessary Cloning

```rust
// ❌ Inefficient for large data
fn process(data: &Vec<Value>) -> BoxStream<'static, Result<Value>> {
    let owned = data.clone();  // Expensive!
    stream::iter(owned.into_iter().map(Ok)).boxed()
}

// ✅ Better - use borrowed lifetime
fn process<'a>(data: &'a Vec<Value>) -> BoxStream<'a, Result<&'a Value>> {
    stream::iter(data.iter().map(Ok)).boxed()
}
```

**Fix:** Use `'a` lifetime when appropriate to avoid cloning.

---

## Quick Reference

### Types Cheat Sheet

```rust
// Type aliases
BoxStream<'static, T>                  = Pin<Box<dyn Stream<Item = T> + Send + 'static>>
BoxStream<'a, T>                       = Pin<Box<dyn Stream<Item = T> + Send + 'a>>
SendableRecordBatchStream             = Pin<Box<dyn RecordBatchStream + Send>>

// Pin variants
Pin<&mut T>                           // Stack pinned reference
Pin<Box<T>>                           // Heap pinned box
```

### Common Operations

```rust
// Creating streams
stream::iter(vec).boxed()                    // From iterator
stream::once(async { value }).boxed()        // Single item
stream::empty().boxed()                      // Empty stream

// Transforming streams
stream.map(|x| x * 2).boxed()               // Transform items
stream.filter(|x| x.is_ok()).boxed()        // Filter items
stream.take(10).boxed()                      // Take first N

// Consuming streams
std::pin::pin!(stream)                       // Pin for iteration
while let Some(item) = stream.next().await   // Iterate
stream.collect::<Vec<_>>().await             // Collect all
```

### Decision Tree

```
Need a stream type?
├─ Returning from function? → Use BoxStream<'static, T>
├─ Borrowing data? → Use BoxStream<'a, T>
├─ Local usage? → Use concrete type
└─ DataFusion? → Use SendableRecordBatchStream

Need to pin?
├─ For iteration? → Use std::pin::pin!(stream)
├─ For storage? → Use Box::pin(stream)
└─ Already pinned? → Use as-is

Need to convert?
├─ Concrete → BoxStream? → Use .boxed()
├─ Stream → RecordBatchStream? → Use Box::pin(adapter)
└─ Add trait object? → Use Box::pin()
```

---

## Summary

**Key Takeaways:**

1. **BoxStream** is shorthand for `Pin<Box<dyn Stream<...>>>`
2. **Pin** prevents memory movement for self-referential types
3. **'static** means owned data, **'a** means borrowed data
4. **Box::pin** and **.boxed()** are equivalent for basic streams
5. **Use 'static** for maximum flexibility unless you need borrowing
6. **std::pin::pin!()** is best for local stream iteration
7. **RecordBatchStream** needs `Box::pin()`, not `.boxed()`

**Best Practices:**

- ✅ Use `BoxStream<'static, T>` for owned streams
- ✅ Use `std::pin::pin!()` for stack pinning
- ✅ Simplify type aliases with BoxStream
- ✅ Use `Box::pin()` for RecordBatchStream adapters
- ❌ Don't forget lifetime parameters
- ❌ Don't confuse Stream and RecordBatchStream traits
- ❌ Don't unnecessarily clone data

---

**For more information:**
- [Rust async book](https://rust-lang.github.io/async-book/)
- [futures crate docs](https://docs.rs/futures/)
- [Pin documentation](https://doc.rust-lang.org/std/pin/)
