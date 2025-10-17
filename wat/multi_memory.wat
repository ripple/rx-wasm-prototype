(module
  ;; --- Multi-Memory Test Case ---
  ;; This module requires the Multi-Memory proposal to validate.
  ;; If the proposal is disabled, the host should fail on validation
  ;; because it encounters multiple memory definitions.

  ;; 1. Define the first memory instance (index 0)
  (memory $mem0 1)

  ;; 2. Define the second memory instance (index 1)
  ;; This line alone should cause a validation error if the proposal is off.
  (memory $mem1 1)

  (func (export "read_from_mem1") (result i32)
    ;; To access memory other than the default (index 0),
    ;; memory access instructions must be prefixed with the memory index.

    ;; 1. Push the address to load (0)
    (i32.const 0)

    ;; 2. Load a 32-bit integer from memory index 1 at address 0.
    ;; The explicit index '1' is the second trigger for failure
    ;; if the proposal is disabled.
    (i32.load 1)
  )

  ;; Initialize the second memory with some data
  (data 1 (i32.const 0) "\01\00\00\00")
)