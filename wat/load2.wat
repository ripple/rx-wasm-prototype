(module
  (memory (export "mem") 1)
  (data (i32.const 0) "\00\00\00\00\00\00\00\01")

  (func (export "finish") (result i64)
    (local $count i32)
    (local $sum i64)

    (local.set $count (i32.const 1000000000))
    (local.set $sum (i64.const 0))

    (loop $my_loop
      ;; Load a 64-bit integer and add it to the sum
      (local.get $sum)
      (i32.const 0)
      (i64.load)
      (i64.add)
      (local.set $sum)

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
    (local.get $sum)
  )
)