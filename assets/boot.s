.extern kernel_main

.global _start
.global stack_top
.global GDT
.global GDT_end


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

.section .data
	gdtr:	.word 0x0
			.long 0x0

.section .text
	_start:
		mov $stack_top, %esp
	
	_gdt:
		xor   %eax, %eax
		lea	  GDT, %eax 
		mov   %eax, (gdtr + 2)
		mov 3, %eax
		mov   %eax, (gdtr)
		LGDT  gdtr


        push %eax
        push %ebx
		cli
		call kernel_main
 
		hang:
			cli      
			hlt      
			jmp hang
