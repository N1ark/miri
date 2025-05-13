//! Regression test for https://github.com/rust-lang/miri/issues/2629
use std::thread;

#[kani::proof]
fn main() {
    thread::scope(|s| {
        s.spawn(|| {});
    });
}
