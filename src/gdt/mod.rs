pub mod access;
pub mod entry;
pub mod flag;

pub use access::*;
pub use entry::*;
pub use flag::*;

static mut GDT: [u64; 2048] = [0; 2048];

