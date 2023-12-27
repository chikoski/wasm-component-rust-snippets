#!/bin/bash

target="wasm32-wasi"
build="release"

build_dir="target/$target/$build"
compose_dir="composed"

function compose() {
    host = $1
    guest = $2
    output = $3
    if [ -n $host ] && [ -n $guest ] && [ -n $output ]; then
        echo "compose $host and $guest"
        cargo component build -r
        mkdir -p $compose_dir
        wasm-tools compose -o "$compose_dir/$output" -d "$build_dir/$guest" "$build_dir/$host" 
    fi
}