#![no_std]

extern crate std as foo;

#[cfg_attr(kani, kani::proof)]
fn main() {}
