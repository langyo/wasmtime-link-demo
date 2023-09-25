use anyhow::Result;

use libloading::*;

#[tokio::main]
async fn main() -> Result<()> {
    let lib = unsafe {
        Library::new({
            #[cfg(target_os = "windows")]
            {
                "./target/release/lib_native.dll"
            }
            #[cfg(target_os = "linux")]
            {
                "./target/release/liblib_native.so"
            }
            #[cfg(target_os = "macos")]
            {
                "./target/release/liblib_native.dylib"
            }
        })
        .expect("Could not load library")
    };
    let func: Symbol<fn(i32) -> i32> = unsafe { lib.get(b"foo").expect("Could not load function") };
    let ret = func(233);
    println!("Answer: {}", ret);

    Ok(())
}
