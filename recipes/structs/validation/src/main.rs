mod schema;
use schema::{MyEnum, MyStruct};
fn main() {
    let my_struct = MyStruct {
        field_1: 1,
        field_2: "valid".to_string(),
        field_3: MyEnum::MyEnumValue(1),
    };

    assert!(my_struct.is_valid());
}
