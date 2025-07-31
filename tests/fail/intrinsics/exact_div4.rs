#![feature(core_intrinsics)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    // division of MIN by -1
    unsafe { std::intrinsics::exact_div(i64::MIN, -1) }; //~ ERROR: overflow in signed remainder (dividing MIN by -1)
}
