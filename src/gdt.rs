use core::arch::asm;

use super::*;

static GDT_LIMIT: usize = 2048;
static mut GDT: [u64; GDT_LIMIT] = [0; GDT_LIMIT];

#[allow(static_mut_refs)]
pub unsafe fn init() -> bool {
    GDT[0] = 0;

    unsafe { GDT[1] = 0x00CF9A000000FFFF }
    unsafe { GDT[2] = 0x00CF92000000FFFF }
    // unsafe { GDT[3] = 0x00CF9A000000FFFF }
    // unsafe { GDT[4] = 0x00CF9A000000FFFF }

    let mut a: *const u32 = ptr::null_mut();
    // set DS to 0

    a = GDT.as_ptr() as *const u32;
    let mut e: u64 = (2048 << 32) | a as u64;
    let mut addr: *const u64 = &e;
    addr = addr.add(2);
    asm!("lgdt [{}]",in(reg) addr);

    let mut res: u64 = 0;
    unsafe {
        // Inline assembly to execute the SGDT instruction and store the result in the gdtr structure
        asm!(
            "sgdt [{}]",  // Store the GDTR (Global Descriptor Table Register) in memory
            in(reg) &mut res, // Use `&mut gdtr` to store the GDTR value at the address of gdtr
        );
    }

    res < e
}
