//@compile-flags: -Zmiri-ignore-leaks
use std::mem;
use std::sync::Mutex;

#[kani::proof]
fn main() {
    // Test for https://github.com/rust-lang/rust/issues/85434
    let m = Mutex::new(5i32);
    mem::forget(m.lock());
}
