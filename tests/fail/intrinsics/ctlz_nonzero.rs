#![feature(core_intrinsics)]

#[cfg_attr(kani, kani::proof)]
pub fn main() {
    unsafe {
        use std::intrinsics::*;

        ctlz_nonzero(0u8); //~ ERROR: `ctlz_nonzero` called on 0
    }
}
