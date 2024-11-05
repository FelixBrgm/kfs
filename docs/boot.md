### GRUB
The **GR**and **U**nified **B**ootloader implements a specification called Multiboot, which is a set of conventions for how a kernel should get loaded into memory. By following its specifications, we can let GRUB load our kernel.
We achieve this using something called a "header". It contains information following the Multiboot format, which GRUB will read and follow.
### Multiboot Header 
`boot.s`:
```asm
.set MB_MAGIC, 0x1BADB002						;      
.set MB_FLAGS, 0
.set MB_CHECKSUM, (0 - (MB_MAGIC + MB_FLAGS))

.section .multiboot								
	.align 4 
	.long MB_MAGIC
	.long MB_FLAGS
	.long MB_CHECKSUM
```

`section .multiboot_header`: Section annotation, needed by the linker to find the boot code.

`.set MB_MAGIC, 0x1BADB002`: 
* `.set MB_MAGIC, `: 
* `0x1BADB002`: The Multiboot specification requires this number be right at the start of the boot code. It is an arbitrary magic number - no need to think about it further.

`.set MB_FLAGS, 0`:
* `0`: Specifies the boot mode (protected).

`dd header_end - header_start`:
* Multiboot expects this value to be the header length, so we use pointer arithmetic to let the CPU calculate it.

`dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))`:
* **checksum**, ensures all values are what they are expected to be. 
	* `0x100000000` overflows to `0`

`dw 0`:
* End tag type.

`dw 0`
* End tag flags.

`dd 8`
* End tag size.

This header then needs to be assembled with `nasm`:
```sh
nasm -f elf64 multiboot_header.s
```
`-f elf64`: Specifies ELF output file format, i.e how framework defining how exactly the bits will be laid out in the object file, `multiboot_header.o`.
### Kernel Code
**Note:** Building with no main is a Rust Nightly feature!
`main.rs`:
```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	loop{}
}
```

Since we are compiling for kernel space, where there is no such thing as a _main function_, we specify `#![no_main]` as global attribute, letting Cargo know not to look for it. Instead, we have the `_start()` symbol, which we will later reference in the [linking process](#linking). Rust also requires us to have a panic handler, defined with the `#[panic_handler]` local attribute. 

`Cargo.toml`:
```toml
[package]
name = "kfs"
version = "0.1.0"
edition = "2021"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Nothing crazy here, just specifying the panic behavior, since it will not be set by default due to `#![no_std]`.

`.cargo/config.toml`:
```toml
[unstable]
build-std = ["core"]

[build]
target = "x86_64-unknown-none"
```

This is our Cargo config, specifying which crates are to be included into our binary. The `[build]` directive sets the build's target architecture, `x86_64`, and `unkown-none` since we are building a custom kernel.

We will need to [link](#linking) our [multiboot header](#multiboot-header) and our actual kernel code into one final executable - we need cargo to give us an object file (`.o`).  This can be achieved using Cargo's rustc sub-module, and telling it to emit mentioned object file:
```sh
cargo rustc --release -- -emit=obj
```

This will output our object file into `target/x86_64-unknown-none/release/deps/kfs-*.o`.
### Linking
We will now link `multiboot_header.o` and `kfs-*.o` into one binary. For this, we will create a linker script.

`linker.ld`:

```ld
ENTRY (_start)

SECTIONS
{
	. = 1M;

	.boot :
	{
		*(.multiboot_header)
	}

	.text :
	{
		*(.text)
	}
}
```

`ENTRY (_start)`:
* Sets the "entry point" of the executable, same symbol as in `main.rs`.

`SECTIONS`:
* Describes where different sections of the binary need to go.

`. = 1M;`
* We start putting sections from the one MB mark. Below that is stuff you do not want to touch..

```
.boot : 
{
	*(.multiboot_header)
}
```

* Creates a section named `boot` and put every section named `multiboot_header` inside of it for grub to see it.

```
.text :
{
	*(.text)
}
```

* This is where the code goes.

We can now use this script to link our binaries like follows:

```sh
ld --nmagic --output=kernel --script=linker.ld multiboot_header.o kfs-*.o
```

`--nmagic`:
* Turns off automatic page alignment, reducing the binary file's size.

### ISO
#### ISO File System Structure
```
iso/
|___boot
   |___grub
   |   |___grub.cfg
   |___kernel
```

`iso/boot/grub/grub.cfg`:

```cfg
set timeout=0
set default=0

menuentry "kfs" {
	multiboot2 /boot/kernel
	boot
}
```

This file configures GRUB. GRUB lets us load different operating systems, displaying a menu of choices on boot. Each `menuentry` is one of those choices.

`iso/boot/kernel` is just the binary we created in the linking process.

We can now create our `.iso` file using `grub-mkrescue`:

```sh
grub-mkrescue -o kfs.iso ./iso
```

### Running the Kernel
To run our kernel, we use QEMU. QEMU is a system emulator. Run:

```sh
qemu-system-x86_64 -cdrom kfs.iso -boot d -nographic
```

`x86_64`:
* Specifies the QEMU variant.

`-cdrom kfs.iso`:
* Start QEMU with CD-ROM drive, its contents being `kfs.iso`.

`-boot d`
* Makes QEMU boot from CD-ROM directly instead of trying Hard Disk and Floppy first.

`-nographic`:
* Runs the kernel in text mode.

#### Resources
* https://www.qemu.org/
* [Multiboot Spec](https://nongnu.askapache.com/grub/phcoder/multiboot.pdf)
* https://intermezzos.github.io/book/first-edition/multiboot-headers.html
