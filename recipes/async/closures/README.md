# Problem
You want to understand how closures work in Rust, both sync and async, especially the difference between borrowing and taking ownership.

# Solution
The code demonstrates several key patterns:

## Sync Closures
- **Borrowing**: `|| { ... }` - closure borrows from scope
- **Ownership**: `move || { ... }` - closure takes ownership

## Async Closures
- **Borrowing**: `|| async { ... }` - async block borrows from scope
- **Ownership**: `|| async move { ... }` - async block takes ownership
- **Reusable**: Use `Arc` for multiple calls with owned data
- **Single-use**: `move || async move` - moves data on first call
- **With spawn**: Closures need `'static` lifetime to be spawned

# Notes
- For reusable async closures with owned data, use `Arc` or clone inside the closure
- The `move` keyword on the closure captures variables from the outer scope
- The `async move` captures variables from wherever the async block is defined
