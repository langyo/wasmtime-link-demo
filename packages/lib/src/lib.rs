extern "C" {
    pub fn outside_func(i: i32) -> i32;
}

pub fn foo(i: i32) -> i32 {
    unsafe { outside_func(i) }
}
