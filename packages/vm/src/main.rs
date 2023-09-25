use anyhow::Result;

use wasmtime::*;

#[tokio::main]
async fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(
        &engine,
        "./target/wasm32-unknown-unknown/release/plugin.wasm",
    )?;

    println!("Module imports: {:?}", module.imports().collect::<Vec<_>>());

    let mut linker = Linker::new(&engine);
    linker.func_wrap("tairitsu", "outside_func", |i: i32| -> i32 {
        println!("outside_func: {}", i);
        i * 114514
    })?;

    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module)?;

    let inside_func = instance.get_typed_func::<i32, i32>(&mut store, "bar")?;
    let result = inside_func.call(&mut store, 233)?;
    println!("Answer: {:?}", result);

    Ok(())
}
