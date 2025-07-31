#![feature(core_intrinsics)]

#[cfg_attr(kani, kani::proof)]
fn main() {
    std::intrinsics::breakpoint(); //~ ERROR: trace/breakpoint trap
}
