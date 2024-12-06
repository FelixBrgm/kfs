mod cursor;
pub mod ps2;
#[allow(clippy::module_inception)]
mod screen;
mod terminal;
pub mod vga;

pub use terminal::Terminal;
