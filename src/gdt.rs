use core::arch::asm;

use super::*;

static GDT_LIMIT: usize = 2048;
static mut GDT: [u64; GDT_LIMIT] = [0; GDT_LIMIT];

#[allow(static_mut_refs)]
pub fn init() {



}