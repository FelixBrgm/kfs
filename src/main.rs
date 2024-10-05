#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_infoo: &PanicInfo) -> ! {
    // Handle the panic: For example, you could print or log the panic info.
    // Since this is a kernel or bare-metal environment, you might want to halt the system.
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Call our kernel initialization function (could include logging, etc.)
    kernel_main();

    loop {}
}

pub fn kernel_main() {}
