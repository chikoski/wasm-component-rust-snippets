#!/bin/bash

wasm="add.wasm"

script_dir=$(cd $(dirname $0); pwd)
project_root=$(dirname $(dirname $script_dir))
wasm_dir="$project_root/composed"
wasm_file="$wasm_dir/$wasm"

wasmtime -W component-model $wasm_file 