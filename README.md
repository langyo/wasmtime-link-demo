# WASMTIME 模块链接示例

前置研究项目，用于验证 WASMTIME 运行 WASM 模块时的可交互性。

## 前置准备

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-make
```

## 调试（WASM 模块）

```bash
cargo make run
```

## 调试（本地模块）

```bash
cargo make run-native
```
