(module
  ;; A basic memory definition (required for any load/store)
  (memory (export "mem") 1)

  ;; The exported function named "finish"
  (func (export "finish") (result i32)
    ;; 1. Push address 0 onto the stack (operand for the load)
    (i32.const 0)

    ;; 2. The core test: i32.atomic.load (Opcode 0xFE 0x10)
    ;; This instruction is exclusive to the Threads Proposal.
    (i32.atomic.load)

    ;; 3. Drop the result (i32) to satisfy the function signature's return value
    (drop)

    ;; 4. Return success signal
    (i32.const 1)
  )
)