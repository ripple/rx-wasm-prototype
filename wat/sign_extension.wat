(module
  ;; --- Sign-Extension Operations Proposal Test Case ---

  ;; This module tests the dedicated Sign-Extension instruction 'i32.extend8_s',
  ;; which was added via a separate proposal, NOT the MVP.
  ;; If the proposal is DISABLED, the host must fail on the 'i32.extend8_s' instruction.

  (func (export "finish") (result i32)
    ;; 1. Push a 32-bit integer with the lower 8 bits set to 0xFF (255).
    (i32.const 0xFF)

    ;; 2. Execute the dedicated sign-extension instruction:
    ;; This instruction takes the i32 value (255) and treats it as an i8.
    ;; The sign bit (the 7th bit of the LSB) is 1, so it sign-extends it,
    ;; resulting in 0xFFFFFFFF (i32 value of -1).
    ;;
    ;; If the host rejects this instruction, the proposal is disabled.
    (i32.extend8_s)
  )
)
