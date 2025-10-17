(module
  ;; Function type: (func (param i32) (result i32))
  (func (export "finish") (param $p i32) (result i32)
    ;; 1. Push the i32 parameter onto the stack
    (local.get $p)
    ;; 2. The core test: Use the i31.new instruction (Opcode 0xFB 0x05)
    (i31.new)
    ;; 3. Drop the resulting i31ref (GC reference type)
    (drop)
    ;; 4. Return success code
    (i32.const 1)
  )
)