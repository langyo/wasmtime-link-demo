pub use lib::foo;

#[no_mangle]
pub extern "C" fn bar(i: i32) -> i32 {
    foo(i)
}

pub fn main() {}
