all:
	cargo build --release
	cp target/x86_64-unknown-none/release/kfs iso/boot/kernel
	grub-mkrescue -o kfs.iso ./iso
	qemu-system-x86_64 -cdrom kfs.iso -boot d -nographic

.PHONY: all