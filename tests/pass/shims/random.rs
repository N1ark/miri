#![feature(random)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _x: i32 = std::random::random(..);
}
