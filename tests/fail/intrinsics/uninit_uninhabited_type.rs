#![feature(never_type)]

#[allow(deprecated, invalid_value)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    let _ = unsafe { std::mem::uninitialized::<!>() }; //~ERROR: constructing invalid value
}
