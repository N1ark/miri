#![feature(random)]

#[kani::proof]
fn main() {
    let _x: i32 = std::random::random();
}
