pub use lib;

pub fn main() {
    let str = "23333333333333\0".to_string();
    let ptr = str.as_ptr();
    let len = str.len();
    unsafe {
        std::ptr::copy_nonoverlapping(ptr, lib::IO_HEAP.as_mut_ptr(), len);

        lib::ready();
    }
}
