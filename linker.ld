/* kernel entrypoint */
ENTRY (_start)


SECTIONS
{
    /* convention -> start writing at 1MB mark */
    . = 1M;

    .boot :
    {
        /* boot.s (assembled to boot.o) */
        *(.multiboot_header)
    }

    .text :
    {
        /* compiled rust object file */
        *(.text)
    }
}