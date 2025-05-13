// This should fail even without validation
//@compile-flags: -Zmiri-disable-validation

#![feature(never_type)]

struct Human;

#[kani::proof]
fn main() {
    let _x: ! = unsafe {
        std::mem::transmute::<Human, !>(Human) //~ ERROR: entering unreachable code
    };
}
