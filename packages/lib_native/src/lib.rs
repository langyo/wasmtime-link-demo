#[no_mangle]
pub extern "C" fn foo(i: i32) -> i32 {
    println!("Hello from dynamic library!");
    i * 114514
}
