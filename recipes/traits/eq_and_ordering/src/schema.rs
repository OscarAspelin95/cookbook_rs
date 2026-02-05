use std::fmt;

/// Trait that can be shared between geometric objects with an area that can be calculated.
pub trait Area {
    fn area(&self) -> usize;
}

impl fmt::Debug for dyn Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Area({})", self.area())
    }
}

// Triangle
#[derive(Debug)]
pub struct Triangle {
    pub base: usize,
    pub height: usize,
}

impl Area for Triangle {
    fn area(&self) -> usize {
        (self.base * self.height) / 2
    }
}

impl<T: Area> PartialEq<T> for Triangle {
    fn eq(&self, other: &T) -> bool {
        self.area() == other.area()
    }
}

impl<T: Area> PartialOrd<T> for Triangle {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// Square
#[derive(Debug)]
pub struct Square {
    pub side: usize,
}

impl Area for Square {
    fn area(&self) -> usize {
        self.side.pow(2)
    }
}

impl<T: Area> PartialEq<T> for Square {
    fn eq(&self, other: &T) -> bool {
        self.area() == other.area()
    }
}

impl<T: Area> PartialOrd<T> for Square {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}
