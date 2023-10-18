pub use lib;

pub fn main() {
    unsafe {
        let str = "Hello from dynamic library!\0".to_string();
        std::ptr::copy_nonoverlapping(str.as_ptr(), lib::IO_HEAP.as_mut_ptr(), str.len());

        lib::ready();
    }
}
