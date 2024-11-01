use core::include;

#[cfg(feature = "build-script")]
include!("build-script.rs");

#[cfg(not(feature = "build-script"))]
fn main() {}
