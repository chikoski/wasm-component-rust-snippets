#!/bin/bash

host="grep-cli.wasm"
guest="filter_id.wasm"
output="grep-id.wasm"

source "./scripts/compose.sh"

compose $host $guest $output