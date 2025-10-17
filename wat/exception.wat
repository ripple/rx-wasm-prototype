(module
  ;; --- Exception Handling Proposal Test ---

  ;; 1. Define a Tag (required by the proposal for exceptions)
  ;; The presence of 'tag' or the instruction 'try' will fail validation
  ;; if the Exception Handling proposal is disabled.
  (tag $t (type (func (result i32))))

  (type $t_finish (func (result i32)))

  ;; Function 'finish'
  (func (export "finish") (type $t_finish) (result i32)
    ;; Push a dummy i32 value (0) onto the stack
    (i32.const 0)

    ;; The core instruction for this test: 'try' (opcode 0x06)
    (try $l0 (result i32)
      ;; Body of the try block: return 1 (success)
      (i32.const 1)
      (return)
    )
    ;; If the 'try' fails compilation, the proposal is disabled.

    ;; Should be unreachable if 'try' is valid and returns.
    (unreachable)
  )
)