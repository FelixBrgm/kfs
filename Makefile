SRC_DIR := src
OBJ_DIR := obj
BUILD_DIR := build
NAME := kernel
BINARY := $(NAME).bin
ISO := $(NAME).iso
MULTIBOOT_HEADER := assets/boot.s
MULTIBOOT_HEADER_OBJ := boot.o

LIB := target/i386-unknown-none/release/libkfs.a

all: $(BUILD_DIR)/$(BINARY)

$(BUILD_DIR)/$(BINARY): $(BUILD_DIR)/$(MULTIBOOT_HEADER_OBJ) $(LIB)
	ld -m elf_i386 -T assets/linker.ld -o $@ $^

$(BUILD_DIR)/$(MULTIBOOT_HEADER_OBJ): $(MULTIBOOT_HEADER) | $(BUILD_DIR)
	as --32 -o $@ $<

$(LIB):
	cargo build-kernel

$(BUILD_DIR):
	mkdir -p $@

run:
	qemu-system-i386 -kernel $(BUILD_DIR)/$(BINARY)

iso: all 
	mkdir -p $(BUILD_DIR)/iso/boot/grub
	cp assets/grub.cfg $(BUILD_DIR)/iso/boot/grub
	cp $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/iso/boot/
	grub-mkrescue -v -o $(BUILD_DIR)/$(NAME).iso $(BUILD_DIR)/iso --compress=xz

cdrom:
	qemu-system-i386 -cdrom $(BUILD_DIR)/$(NAME).iso -boot d

fclean:
	cargo clean
	$(RM) -rf $(BUILD_DIR)

re: fclean all

.PHONY: all run re fclean iso cdrom
