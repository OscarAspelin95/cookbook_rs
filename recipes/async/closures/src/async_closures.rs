pub async fn async_closure_that_borrows(s: String) {
    let closure = || async {
        println!("Borrowed in async block: {s}");
    };
    closure().await;
    println!("`s` still available after async closure: {s}");
}

/// Async closure that TAKES OWNERSHIP via `async move`.
/// Pattern: || async move { ... }
/// The `async move` block captures `s` by moving it into the async block.
pub async fn async_closure_that_takes_ownership(s: String) {
    let closure = || async move {
        println!("Owned in async move block: {s}");
    };
    closure().await;
    // println!("{s}"); // ERROR: `s` was moved into the async block
}

/// For REUSABLE async closures, use Arc or clone inside the closure.
/// Pattern: move || async move with Arc<T>
/// Use Arc for shared ownership across multiple calls.
pub async fn async_closure_reusable(s: String) {
    use std::sync::Arc;
    let s = Arc::new(s);

    let closure = move || {
        let s = Arc::clone(&s);
        async move {
            println!("Reusable async closure: {s}");
        }
    };

    // Can call multiple times because each call clones the Arc
    closure().await;
    closure().await;
    closure().await;
}

/// Pattern: move || async move { ... }
/// The outer `move` captures `s` into the closure.
/// The inner `async move` moves `s` from closure into the async block on FIRST call.
/// Cannot call again because the value was moved out of the closure.
pub async fn async_closure_single_use(s: String) {
    let closure = move || async move {
        println!("Single-use async closure: {s}");
    };

    closure().await;
    // closure().await; // ERROR: `s` was moved out of closure on first call
}

/// Using `move || async move` for 'static lifetime requirements.
/// Pattern: move || async move { ... }
pub async fn async_closure_with_spawn(s: String) {
    let closure = move || async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("Spawned task with: {s}");
    };

    // The closure needs to own its data to be spawned
    tokio::spawn(closure());
}
