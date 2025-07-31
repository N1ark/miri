//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
#![feature(extern_types)]

use std::ptr;

extern "C" {
    type Foo;
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    let x: &Foo = unsafe { &*(ptr::without_provenance::<()>(16) as *const Foo) };
    let _y: &Foo = &*x;
}
