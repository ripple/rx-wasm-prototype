;;(module
;;  ;; This function is designed to test the activation of the "Reference-Typed Strings Proposal".
;;  ;; If the proposal is disabled (as desired by the user's configuration),
;;  ;; the WASM validation/compilation step in wasmi should fail because:
;;  ;; 1. The result type 'stringref' is unknown.
;;  ;; 2. The instruction 'string.const' is unknown.
;;
;;  (type $t_string (func (result stringref)))
;;
;;  (func (export "get_constant_string") (type $t_string)
;;    ;; This instruction is the core test. It creates a reference-typed string
;;    ;; from a literal value.
;;    (string.const "WASM String Ref Test")
;;  )
;;)
(module
  ;; --- Header and Version (0061736d01000000) ---

  ;; Type Section (01)
  (type $t0 (func (result stringref)))
  ;; 01 05 01 60 00 01 6e (Type $t0 returns stringref (0x6e))

  ;; Import Section (02)
  ;; This module implicitly imports the string built-in feature set,
  ;; usually via the "string" module (02 04 01 20 00) in the reference implementation.

  ;; Function Section (03)
  (func $f0 (type $t0))
  ;; 03 02 01 00

  ;; Export Section (07)
  (export "finish" (func $f0))
  ;; 07 08 01 04 66 69 6e 69 73 68 00 00

  ;; Code Section (0a)
  (func $f0
    ;; Instruction: string.const (0xfc 0x18) followed by a string index (0x00)
    (string.const 0)
    ;; fc 18 00
    (drop)
    ;; 1a
    (i32.const 1)
    ;; 41 01
    (unreachable)
    ;; 00
    (end)
    ;; 0b
  )
)

should convertable to:
0061736d010000000105016000016e030201000708010466696e69736800000a0a010800fc18000a450b
But wat2wasm report error. So tested the hex directly. 
