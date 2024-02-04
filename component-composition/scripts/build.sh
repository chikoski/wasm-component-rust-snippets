#!/bin/bash

host="calculation-cli.wasm"
guest="calculation_add.wasm"
output="add.wasm"

script_dir=$(cd $(dirname $0); pwd)
project_root=$(dirname $(dirname $script_dir))

source "$project_root/scripts/compose.sh"

cargo component build -r
compose $host $guest $output
