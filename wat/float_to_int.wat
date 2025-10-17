(module
  ;; --- Non-trapping Float-to-Int Conversions Proposal Test ---

  ;; This module tests for the 'i32.trunc_sat_f32_s' instruction,
  ;; which is exclusive to the Non-trapping Float-to-Int Conversions Proposal.
  ;; If this proposal is disabled, validation will fail.

  (type (func (param f32) (result i32)))

  ;; Function 'finish' attempts a saturating conversion.
  ;; It takes an f32, clamps it to the i32 range if necessary, and returns the result.
  (func (export "finish") (param $val f32) (result i32)
    (local.get $val)

    ;; This instruction performs the non-trapping conversion.
    ;; If $val is 1.0, result is 1.
    ;; If $val is 3.0e38 (overflow), result is 2147483647 (i32 max).
    ;; If the host rejects this instruction, the proposal is confirmed disabled.
    (i32.trunc_sat_f32_s)
  )
)