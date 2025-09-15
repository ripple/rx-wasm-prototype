(module
  ;; A function that loops a fixed number of times and performs a division.
  (func (export "finish") (result i32)
    (local $count i32)
    (local.set $count (i32.const 100000000))

    (loop $my_loop
      ;; The operation we are measuring: i64.div_s
      (i64.const 10)
      (i64.const 2)
      (i64.div_s)
      (drop) ;; Consume the result to keep the stack balanced

      ;; Decrement the counter
      (local.get $count)
      (i32.const 1)
      (i32.sub)
      (local.set $count)

      ;; Repeat the loop if the counter is not zero.
      (local.get $count)
      (i32.const 0)
      (i32.ne)
      (br_if $my_loop)
    )

    (i32.const 0) ;; Return a dummy value
  )
)