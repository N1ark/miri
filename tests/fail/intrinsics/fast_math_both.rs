#![feature(core_intrinsics)]

fn main() {
    unsafe {
        let _x: f32 = std::intrinsics::fsub_fast(f32::NAN, f32::NAN); //~ ERROR: `fsub_fast` intrinsic called with non-finite value as both parameters
    }
}
