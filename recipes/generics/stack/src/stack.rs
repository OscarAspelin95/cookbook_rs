use std::fmt::{Debug, Display};

/// This is basically just a wrapper around a vec, but it still a good
/// beginner excercise for handling a generic type with trait bounds in a struct.
#[derive(Debug)]
pub struct Stack<T>
where
    T: Debug + Display + Ord + ToString,
{
    values: Vec<T>,
}

impl<T: Debug + Display + Ord + ToString> Stack<T> {
    pub fn new() -> Self {
        let values: Vec<T> = Vec::new();
        Self { values }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let values: Vec<T> = Vec::with_capacity(capacity);
        Self { values }
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.values.first()
    }

    pub fn sort(&mut self) {
        self.values.sort()
    }

    fn values_str(&self) -> String {
        self.values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("->")
    }
}

impl<T> Display for Stack<T>
where
    T: Ord + Debug + ToString + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values_str())
    }
}
