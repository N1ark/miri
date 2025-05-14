#![feature(core_intrinsics)]

fn main() {
    unsafe {
        let _x: f32 = std::intrinsics::fmul_fast(3.4f32, f32::INFINITY); //~ ERROR: `fmul_fast` intrinsic called with non-finite value as second parameter
    }
}
