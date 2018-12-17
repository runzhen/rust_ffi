#![crate_type = "dylib"]

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    println!("hello --from rust shared library");
    input * 2
}
