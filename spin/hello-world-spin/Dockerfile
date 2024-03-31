FROM scratch
COPY spin.toml /spin.toml
COPY target/wasm32-wasi/release/hello_world_spin.wasm /target/wasm32-wasi/release/hello_world_spin.wasm
ENTRYPOINT ["/spin.toml"]