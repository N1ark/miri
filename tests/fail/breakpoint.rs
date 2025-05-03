#![feature(core_intrinsics)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    core::intrinsics::breakpoint(); //~ ERROR: trace/breakpoint trap
}
