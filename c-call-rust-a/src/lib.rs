#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    println!("hello --from rust static library");
    input * 2
}
