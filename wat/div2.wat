(module
  (func (export "finish") (result i64)
    (local $count i32)
    (local $accumulator i64)

    (local.set $count (i32.const 100000000))
    (local.set $accumulator (i64.const 100000000)) ;; Initial value to prevent division by zero

    (loop $my_loop
      ;; Perform the division and accumulate the result
      (local.get $accumulator)
      (i64.const 2)
      (i64.div_s)
      (local.set $accumulator)

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
    (local.get $accumulator) ;; Return the final value
  )
)