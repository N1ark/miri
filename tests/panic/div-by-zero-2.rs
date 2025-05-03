#![allow(unconditional_panic)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _n = 1 / 0;
}
