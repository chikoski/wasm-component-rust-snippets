# Prerequisites

- Rust with wasm32-wasi target.
- cargo-component
- wasmtime-cli

# How to build

Just build with cargo-component as following example:

```zsh
% cargo component build -r
```

# How to run

- This sample assumes that a text file exits as `./src/main.rs`.
- You need grant fs access to the built wasm file by passing `--dir` option to wasmtime.
- component-model feature should be enabled.

```zsh
% wasmtime --dir=.::/ -W component-model target/wasm32-wasi/release/hello-wasi-component.wasm
```