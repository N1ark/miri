#![allow(unconditional_panic)]

#[kani::proof]
fn main() {
    let _n = 1 / 0;
}
