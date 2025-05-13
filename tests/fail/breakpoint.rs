#![feature(core_intrinsics)]

#[kani::proof]
fn main() {
    core::intrinsics::breakpoint(); //~ ERROR: trace/breakpoint trap
}
