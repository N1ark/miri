#![feature(core_intrinsics)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    // division by 0
    unsafe { std::intrinsics::exact_div(2, 0) }; //~ ERROR: divisor of zero
}
