(module
  (memory (export "mem") 1)

  (func (export "finish") (result i32)
    (local $count i32)
    (local.set $count (i32.const 100000000))

    (loop $my_loop
      ;; The operation we are measuring: i64.store
      (i32.const 0)   ;; Push memory offset 0 onto the stack
      (i64.const 42)  ;; Push the value to be stored
      (i64.store)     ;; Store the value at offset 0

      ;; Decrement the counter
      (local.get $count)
      (i32.const 1)
      (i32.sub)
      (local.set $count)

      (local.get $count)
      (i32.const 0)
      (i32.ne)
      (br_if $my_loop)
    )
    (i32.const 0)
  )
)