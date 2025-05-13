#![allow(arithmetic_overflow)]

#[kani::proof]
fn main() {
    let _n = 2i64 << -1;
}
