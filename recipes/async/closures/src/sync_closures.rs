pub fn sync_closure_that_borrows(s: String) {
    let closure = || {
        println!("Borrowed variable `s`: {s}");
    };
    closure();
    println!("`s` available outside of the closure: {s}");
}

/// Closure takes ownership of `s` via `move`, so `s` is NOT available outside.
pub fn sync_closure_that_takes_ownership(s: String) {
    let closure = move || {
        println!("Owned variable: {s}");
    };
    closure();
    // println!("{s}"); // ERROR: `s` was moved into the closure
}
