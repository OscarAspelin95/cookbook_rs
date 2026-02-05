use serde::Serialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize)]
pub enum MyEnum {
    MyEnumValue(usize),
}

#[derive(Debug, Validate, Serialize)]
pub struct MyStruct {
    #[validate(range(min = 1, max = 10))]
    pub field_1: usize,
    #[validate(length(min = 1, max = 10))]
    pub field_2: String,
    // Custom function to validate enum(s).
    #[validate(custom(function = "validate_enum"))]
    pub field_3: MyEnum,
}

impl MyStruct {
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

fn validate_enum(field_3: &MyEnum) -> Result<(), ValidationError> {
    match field_3 {
        MyEnum::MyEnumValue(val) => (*val <= 10)
            .then(|| ())
            .ok_or(ValidationError::new("Value for field 3 must be <= 10")),
    }
}
