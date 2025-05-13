#![allow(arithmetic_overflow)]

#[kani::proof]
fn main() {
    let _n = 1i64 >> 64;
}
