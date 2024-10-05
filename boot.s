mov rsp, 0x00000000 ; Should be the value of the stack 
                    ; Load in the kernel


jmp 0x00000000 ; Set Instruction Pointer to the location of the start of the kernel binary in memory