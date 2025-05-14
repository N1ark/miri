#![feature(core_intrinsics)]

fn main() {
    std::intrinsics::breakpoint(); //~ ERROR: trace/breakpoint trap
}
