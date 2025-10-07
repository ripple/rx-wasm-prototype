(module
;; Define a single memory block with an initial size of 1 page (64KiB)
(memory (export "mem") 1)

;; 1. Initialize data at address 0.
;; The data segment contains the byte sequence: [72, 101, 108, 108, 111, 33] (ASCII for "Hello!")
(data (i32.const 0) "Hello!")

;; Function to perform the copy operation.
;; It copies a block of data from address 0 to address 10, for 6 bytes.
;; After execution, memory[0..5] = "Hello!" and memory[10..15] = "Hello!"
(func (export "do_copy")
;; Stack preparation for memory.copy: [destination_addr, source_addr, length]

;; 1. Push Destination Address (10)
(i32.const 10)

;; 2. Push Source Address (0)
(i32.const 0)

;; 3. Push Length (6 bytes)
(i32.const 6)

;; Execute the bulk memory copy operation
;; should trigger an error
(memory.copy)

)
)