use core::arch::asm;

use super::*;

static GDT_LIMIT: usize = 2048;
static mut GDT: [u64; GDT_LIMIT] = [0; GDT_LIMIT];

#[allow(static_mut_refs)]
pub unsafe fn init() {
    let mut t: Terminal = Terminal::new();

    t.clear_screen();
    GDT[0] = 0;

    unsafe { GDT[1] = 0x00CF9A000000FFFF }
    unsafe { GDT[2] = 0x00CF92000000FFFF }
    // unsafe { GDT[3] = 0x00CF9A000000FFFF }
    // unsafe { GDT[4] = 0x00CF9A000000FFFF }

    let gdt_addr = GDT.as_ptr() as *const u32;
    print_u64(&mut t, gdt_addr as u64);
    t.write_str("|gdt_addr");
    t.new_line();

    // let mut gdt_register: u64 = (2048 << 32) | gdt_addr as u64;

    // print_u64(&mut t, gdt_register as u64);
    // t.new_line();

    // let mut addr: *const u64 = &gdt_register;
    // asm!("lgdt [{}]",in(reg) addr);

    // let mut gdt_reg_res: u64 = 0;
    // unsafe {
    //     // Inline assembly to execute the SGDT instruction and store the result in the gdtr structure
    //     asm!(
    //         "sgdt [{}]",  // Store the GDTR (Global Descriptor Table Register) in memory
    //         in(reg) &mut gdt_reg_res, // Use `&mut gdtr` to store the GDTR value at the address of gdtr
    //     );
    // }

    // print_u64(&mut t, gdt_reg_res);
}

fn print_u64(t: &mut Terminal, n: u64) {
    let res = u64_to_base(n, 16);
    let res = match res {
        Ok((len, buf)) => (len, buf),
        Err(()) => (0, [b'a'; 65]),
    };
    t.write_u8_arr(&res.1);
    t.write_str(core::str::from_utf8(&res.1[65 - res.0..]).unwrap());
}
