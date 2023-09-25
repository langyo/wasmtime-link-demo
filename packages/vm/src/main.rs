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
    /*
        由 rust 编译出来的 wasm 从外部包含模块时，默认模块名为 env
        如果需要更改这里的 module 参数，
        请在对应的 wasm 模块的导出函数（`extern "C"` 内）的声明前
        加上过程宏 `#[link(wasm_import_module = "模块名")]`
    */
    linker.func_wrap("env", "outside_func", |i: i32| -> i32 {
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
