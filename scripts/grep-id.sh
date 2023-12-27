#!/bin/bash

host="grep-cli.wasm"
guest="id_multiline.wasm"
output="grep-id.wasm"

source "./scripts/compose.sh"

compose $host $guest $output