mod schema;
use schema::MyStruct;
fn main() {
    // Vec of type T = i32.
    let mut v_i32 = vec![MyStruct { field_1: 1 }, MyStruct { field_1: 0 }];
    v_i32.sort_by_key(|my_struct| my_struct.field_1);
    println!("{:?}", v_i32);

    // Vec of type T = &str.
    let mut v_str = vec![MyStruct { field_1: "B" }, MyStruct { field_1: "A" }];
    v_str.sort_by_key(|my_struct| my_struct.field_1);
    println!("{:?}", v_str);
}
