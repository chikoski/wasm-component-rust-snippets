(module $canonical_abi.wasm
  (type (;0;) (func (param i32 i32 i32 i32)))
  (type (;1;) (func (param i32) (result i64)))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func))
  (func $new (;0;) (type 0) (param i32 i32 i32 i32)
    local.get 0
    local.get 3
    i32.store16 offset=6
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $hash (;1;) (type 1) (param i32) (result i64)
    local.get 0
    i64.load8_u offset=4
    i64.const 16
    i64.shl
    local.get 0
    i64.load32_u
    i64.const 32
    i64.shl
    i64.or
    local.get 0
    i64.load16_u offset=6
    i64.or
  )
  (func $add (;2;) (type 2) (param i32 i32 i32)
    local.get 0
    local.get 2
    i32.load16_u offset=6
    local.get 1
    i32.load16_u offset=6
    i32.add
    i32.store16 offset=6
    local.get 0
    local.get 2
    i32.load8_u offset=4
    local.get 1
    i32.load8_u offset=4
    i32.add
    i32.store8 offset=4
    local.get 0
    local.get 2
    i32.load
    local.get 1
    i32.load
    i32.add
    i32.store
  )
  (func $dummy (;3;) (type 3))
  (func $__wasm_call_dtors (;4;) (type 3)
    call $dummy
    call $dummy
  )
  (func $new.command_export (;5;) (type 0) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $new
    call $__wasm_call_dtors
  )
  (func $hash.command_export (;6;) (type 1) (param i32) (result i64)
    local.get 0
    call $hash
    call $__wasm_call_dtors
  )
  (func $add.command_export (;7;) (type 2) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $add
    call $__wasm_call_dtors
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "new" (func $new.command_export))
  (export "hash" (func $hash.command_export))
  (export "add" (func $add.command_export))
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.81.0 (eeb90cda1 2024-09-04)")
    (processed-by "clang" "18.1.2 (https://github.com/llvm/llvm-project 26a1d6601d727a96f4301d0d8647b5a42760ae0c)")
  )
  (@custom "target_features" (after code) "\03+\0bbulk-memory+\0fmutable-globals+\08sign-ext")
)
