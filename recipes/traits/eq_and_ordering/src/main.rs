mod schema;
use schema::{Square, Triangle};

use crate::schema::Area;

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

    // We can even sort now based on area.
    let mut v: Vec<Box<dyn Area>> = vec![Box::new(triangle), Box::new(square)];
    v.sort_by(|a, b| a.area().cmp(&b.area()).reverse());
    println!("{:?}", v);
}
