static IO_HEAP: [u8; 8192] = [0; 8192];

extern "C" {
    pub fn ready(address: i32) -> ();
}

#[no_mangle]
pub fn get_heap_address() -> i32 {
    IO_HEAP.as_ptr() as i32
}
