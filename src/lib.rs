#![no_std]

#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];

mod gdt;
mod idt;
mod print;
mod ps2;
mod vga;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut t = vga::Vga::new();
    t.clear_screen();

    loop {
        if let Some(char) = ps2::read_if_ready(None) {
            if char == ps2::BACKSPACE {
                t.delete_char();
            } else if char == ps2::ENTER {
                t.new_line();
            } else if char == ps2::ARROW_LEFT {
                t.move_cursor(vga::Direction::Left);
            } else if char == ps2::ARROW_RIGHT {
                t.move_cursor(vga::Direction::Right);
            } else if char == ps2::ARROW_UP || char == ps2::ARROW_DOWN {
            } else {
                t.write_char(char as u8);
            }
        }
    }
}
