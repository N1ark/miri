#![allow(arithmetic_overflow)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _n = 1i64 >> 64;
}
