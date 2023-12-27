#!/bin/bash

output="ferris_says.wasm"

host="host-says.wasm"
guest="guest_ferris.wasm"

target="wasm32-wasi"
build="release"

build_dir="target/$target/$build"
compose_dir="composed"

cargo component build -r
mkdir -p $compose_dir
wasm-tools compose -o "$compose_dir/$output" -d "$build_dir/$guest" "$build_dir/$host" 