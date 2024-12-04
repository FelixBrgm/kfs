#![no_std]


#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];

mod gdt;
mod idt;
mod print;
mod ps2;
mod terminal;
mod vga;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    start_terminal();
}

pub fn start_terminal() {
    let mut t = terminal::terminal::Terminal::new();
    t.flush();
    loop {
        if let Some(key) = terminal::ps2::read_if_ready() {
            t.handle_key(key);
            t.flush();
        }
    }
}
