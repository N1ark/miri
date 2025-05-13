#![allow(deprecated)]

use std::mem;

struct Foo {
    _inner: mem::MaybeUninit<i32>,
}

#[kani::proof]
fn main() {
    unsafe {
        let foo = Foo { _inner: mem::uninitialized() };
        let _bar = foo;
    }
}
