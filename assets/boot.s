.extern kernel_main

.global _start
.global stack_top


.set MB_MAGIC, 0x1BADB002          
.set MB_FLAGS, 0
.set MB_CHECKSUM, (0 - (MB_MAGIC + MB_FLAGS))

.section .multiboot
	.align 4 
	.long MB_MAGIC
	.long MB_FLAGS
	.long MB_CHECKSUM

.section .bss

	.align 16
	stack_bottom:
		.skip 4096
	stack_top:

 
.section .text
	_start:
		mov $stack_top, %esp


		gdtr DW 0 ; For limit storage
			DD 0 ; For base storage

		XOR   EAX, EAX
		MOV   AX, DS
		SHL   EAX, 4
		ADD   EAX, ''GDT''
		MOV   [gdtr + 2], eax
		MOV   EAX, ''GDT_end''
		SUB   EAX, ''GDT''
		MOV   [gdtr], AX
		LGDT  [gdtr]

        push %eax
        push %ebx
		cli
		call kernel_main
 
		hang:
			cli      
			hlt      
			jmp hang
