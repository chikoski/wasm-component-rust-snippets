(module
  (type (;0;) (func (param i32 i32 i32 i32)))
  (type (;1;) (func (param i32) (result i64)))
  (func $new (;0;) (type 0) (param i32 i32 i32 i32)
    local.get 0
    local.get 2
    i32.store8 offset=6
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 3
    i32.store16 offset=4
  )
  (func $hash (;1;) (type 1) (param i32) (result i64)
    local.get 0
    i64.load8_u offset=6
    i64.const 16
    i64.shl
    local.get 0
    i64.load32_u
    i64.const 32
    i64.shl
    i64.or
    local.get 0
    i64.load16_u offset=4
    i64.or
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "new" (func $new))
  (export "hash" (func $hash))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.74.0 (79e9716c9 2023-11-13)")
  )
)