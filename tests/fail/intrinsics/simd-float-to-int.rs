#![feature(portable_simd)]
use std::simd::prelude::*;

#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        let _x: i32x2 = f32x2::from_array([f32::MAX, f32::MIN]).to_int_unchecked(); //~ERROR: cannot be represented in target type `i32`
    }
}
