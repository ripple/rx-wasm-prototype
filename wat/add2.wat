(module
  ;; A function that loops a fixed number of times and performs an addition.
  ;; The result is to ensure the loop body is not optimized away.
  (func (export "finish") (result i32)
    (local $count i32)
    (local $sum i32)
    (local.set $count (i32.const 1000000000))
    (local.set $sum (i32.const 0))

    (loop $my_loop
      ;; Perform the operation and accumulate the result.
      (local.get $sum)
      (i32.const 1)
      (i32.add)
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

    (local.get $sum) ;; Return the final sum to prevent optimization
  )
)