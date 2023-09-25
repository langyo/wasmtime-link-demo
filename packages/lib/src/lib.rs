#[link(wasm_import_module = "tairitsu")]
#[allow(unused_attributes)]
#[no_mangle]
extern "C" {
    pub fn outside_func(i: i32) -> i32;
}

pub fn foo(i: i32) -> i32 {
    unsafe { outside_func(i) }
}
