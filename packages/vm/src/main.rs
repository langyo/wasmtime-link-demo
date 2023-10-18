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
    linker.func_wrap("env", "ready", || {
        println!("Ready");
    })?;

    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module)?;

    let inside_func = instance.get_typed_func::<(), i32>(&mut store, "get_heap_address")?;
    let result = inside_func.call(&mut store, ())?;
    println!("Get heap address from vm: {:?}", result);

    let main_func = instance.get_typed_func::<(i32, i32), i32>(&mut store, "main")?;
    main_func.call(&mut store, (0, 0))?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or(anyhow::anyhow!("no memory"))?;
    let mut buffer = [0u8; 8192];
    memory.read(&mut store, result as usize, &mut buffer)?;
    let str: String = buffer
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect();
    println!("Get heap string from vm: {}", str);

    Ok(())
}
