all:
	cargo rustc --release -- --emit=obj
	nasm -f elf64 multiboot_header.s
	ld --nmagic --output=kernel --script=linker.ld multiboot_header.o target/x86_64-unknown-none/release/deps/kfs-e34396d0b60553df.o -z noexecstack
	rm multiboot_header.o
	cp kernel iso/boot/kernel
	rm kernel
	grub-mkrescue -o kfs.iso ./iso
	qemu-system-x86_64 -cdrom kfs.iso -boot d -nographic

.PHONY: all