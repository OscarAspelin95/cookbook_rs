mod schema;
use schema::{Square, Triangle};

fn main() {
    let triangle = Triangle {
        base: 10,
        height: 2,
    };
    let square = Square { side: 5 };

    // We have defined Eq as comparing areas.
    assert!(triangle != square);
    assert!(triangle < square);
    assert!(square > triangle);
}
