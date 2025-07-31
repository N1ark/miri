#![feature(core_intrinsics)]

#[cfg_attr(kani, kani::proof)]
pub fn main() {
    unsafe {
        use std::intrinsics::*;

        cttz_nonzero(0u8); //~ ERROR: `cttz_nonzero` called on 0
    }
}
