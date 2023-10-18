pub static mut IO_HEAP: [u8; 8192] = [0; 8192];

extern "C" {
    pub fn ready() -> ();
}

#[no_mangle]
pub fn get_heap_address() -> i32 {
    unsafe { IO_HEAP.as_ptr() as i32 }
}
