(module
  ;; --- Memory64 Test Case ---
  ;; This module requires the Memory64 proposal to validate.
  ;; If the proposal is disabled, the host should fail on validation
  ;; because it encounters the i64 memory address type.

  ;; 1. Define a single memory instance using 64-bit addressing (i64).
  ;; The address space type 'i64' is the primary trigger for this test.
  ;; Syntax: (memory i64 <min_pages>)
  (memory $mem0 i64 1)

  ;; Initialize the data segment at address i64.const 0
  (data (i64.const 0) "\01\00\00\00")

  (func (export "read_from_mem64") (result i32)
    ;; The address instruction must push an i64 value onto the stack,
    ;; matching the memory's addressing mode.
    (i64.const 0)

    ;; Load a 32-bit integer from the i64 address provided.
    (i32.load)
  )
)