#![feature(never_type)]
use std::mem::{forget, transmute};

#[kani::proof]
fn main() {
    unsafe {
        let x: Box<!> = transmute(&mut 42); //~ERROR: encountered a box pointing to uninhabited type !
        forget(x);
    }
}
