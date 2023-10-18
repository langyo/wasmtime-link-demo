pub use lib;

pub fn main() {
    unsafe {
        lib::ready(lib::get_heap_address());
    }
}
