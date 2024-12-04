use core::panic::PanicInfo;

use crate::terminal::Color;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::terminal::Terminal;

    let mut t = Terminal::new();
    for &c in "Paniced!".as_bytes().iter() {
        t.write_color(c, Color::Error as u8);
    }
    t.flush();
    loop {}
}
