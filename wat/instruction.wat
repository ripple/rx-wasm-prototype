(module
;; Exported function "finish" returns the constant integer 33.
(func (export "finish") (result i32)
(i32.const 33)
)

;; Define a memory block (required by many environments, even if unused)
(memory (export "mem") 1)
)