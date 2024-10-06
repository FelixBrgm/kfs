section .multiboot_header

header_start: ; label -> allows to refer to specific part of code
    ; dd -> define double word (word: sequence of 16 bits, like in networking)
    dd 0xe85250d6                   ; magic number, completely arbitrary (try  if it does not work)
    dd 0                            ; protected mode
    dd header_end - header_start    ; header length

    ; checksum -> 0x100000000 means unsigned 
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dw 0    ; size
header_end:
