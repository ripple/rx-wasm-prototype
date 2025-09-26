(module
  ;; Define a memory with a min of 1 page and a max of 10 pages.
  ;; This ensures the memory.grow instruction will eventually fail.
  (memory (export "mem") 1 10)

  (func (export "finish") (result i32)
    (local $count i32)
    (local $dummy_result i32)

    ;; Set the loop count to 100,000,000
    (local.set $count (i32.const 100000000))

    (loop $my_loop
      ;; The operation we are measuring: memory.grow
      ;; It will eventually start failing as it reaches the limit.
      (i32.const 1)  ;; Grow by 1 page
      (memory.grow)
      (local.set $dummy_result)

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

    (local.get $dummy_result)
  )
)