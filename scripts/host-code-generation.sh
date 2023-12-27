#!/bin/bash

output="ferris_says.wasm"

host="host-says.wasm"
guest="guest_ferris.wasm"

source ./scripts/compose.sh

compose $host $guest $output