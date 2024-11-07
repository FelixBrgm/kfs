pub mod access;
pub mod entry;
pub mod flag;
pub mod init;

pub use access::*;
pub use entry::*;
pub use flag::*;
pub use init::*;

static mut GDT: [u64; 2048] = [0; 2048];
