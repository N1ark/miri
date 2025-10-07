use std::{fmt, mem};

#[cfg_attr(kani, kani::proof)]
fn main() {
    let x: &dyn Send = &0;
    let _y: *const dyn fmt::Debug = unsafe { mem::transmute(x) }; //~ERROR: wrong trait
}
