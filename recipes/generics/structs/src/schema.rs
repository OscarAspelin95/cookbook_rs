use std::fmt::Debug;

#[derive(Debug)]
pub struct MyStruct<T: PartialOrd + Debug> {
    pub field_1: T,
}
