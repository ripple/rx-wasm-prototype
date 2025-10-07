(module
;; This function is exported so it can be called from the host.
;; It's named "finish".
(func (export "finish")

;; Define the return value type as i32.
(result i32)

;; --- Function Body ---

;; 1. Perform an f32 addition. This is the operation we are testing.
f32.const 10.5
f32.const 20.0
f32.add
;;i32.trunc
;; The f32 result (30.5) is now on the top of the stack.

;; 2. Discard the f32 result from the stack.
drop

;; 3. Push a hard-coded i32 constant onto the stack.
;; This will be the function's return value, matching the (result i32) signature.
i32.const 42

)
)