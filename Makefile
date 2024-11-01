all:
	cargo build-kernel 
	cargo build-iso
	# qemu-system-x86_64 -cdrom kfs.iso -boot d -nographic

fclean:
	cargo clean
	rm ./iso/boot/kernel
	rm kernel.iso
	find . -name multiboot_header.o -delete

.PHONY: all