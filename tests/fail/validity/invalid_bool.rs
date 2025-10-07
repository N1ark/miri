#![allow(unnecessary_transmutes)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    let _b = unsafe { std::mem::transmute::<u8, bool>(2) }; //~ ERROR: expected a boolean
}
