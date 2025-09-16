(module
  ;; A memory with a minimum of 1 page (64KB), exported for the host.
  (memory (export "mem") 1)

  ;; Initialize the memory with some data at byte offset 0.
  ;; The data will be 64-bit integers: 1234, 5678...
  (data (i32.const 0) "\00\00\00\00\00\00\04\D2\00\00\00\00\00\00\16\2E")

  (func (export "finish") (result i32)
    (local $count i32)
    (local.set $count (i32.const 100000000))

    (loop $my_loop
      ;; The operation we are measuring: i64.load
      (i32.const 0) ;; Push memory offset 0 onto the stack
      (i64.load)    ;; Load a 64-bit integer from offset 0
      (drop)        ;; Consume the result

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