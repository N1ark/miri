#![allow(arithmetic_overflow)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _n = 2i64 << -1;
}
