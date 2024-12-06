use core::panic::PanicInfo;

use crate::terminal::vga::Color;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::terminal::Terminal;

    let mut t = Terminal::default();
    t.write_color_str("Paniced!", Color::Error as u8);
    t.flush();
    loop {}
}
