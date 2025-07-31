#![allow(invalid_value)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _b: fn() = unsafe { std::mem::transmute(0usize) }; //~ ERROR: encountered a null function pointer
}
