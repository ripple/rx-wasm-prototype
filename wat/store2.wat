(module
  (memory (export "mem") 1)

  (func (export "finish") (result i32)
    (local $count i32)
    (local $value i64)

    (local.set $count (i32.const 1000000000))
    (local.set $value (i64.const 0))

    (loop $my_loop
      ;; Perform the store operation and update the stored value
      (i32.const 0)
      (local.get $value)
      (i64.store)

      (local.get $value)
      (i64.const 1)
      (i64.add)
      (local.set $value)

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