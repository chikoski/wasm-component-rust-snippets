#!/bin/bash

target=${target:="wasm32-wasi"}
build=${build:="release"}

script_dir=${script_dir:=$(cd $(dirname $0); pwd)}
project_root=${project_root:=$(dirname $script_dir)}

build_dir="$project_root/target/$target/$build"
compose_dir="$project_root/composed"

function compose() {
    host=$1
    guest=$2
    output=$3

    echo "Project root = $root"
    echo "Composing Wasm files in $build_dir"
    if [ -n $host ] && [ -n $guest ] && [ -n $output ]; then
        do_compose $host $guest $output
    fi
}

function do_compose() {
    host=$1
    guest_original=$2
    output=$3

    echo "compose $host and $guest_original"
    cd $build_dir
    guest=${guest_original//_/-}
    if [ $guest_original != $guest ]; then
        mv $guest_original $guest
    fi
    mkdir -p $compose_dir
    wasm-tools compose -o "$compose_dir/$output" -d "$build_dir/$guest" "$build_dir/$host"     
}