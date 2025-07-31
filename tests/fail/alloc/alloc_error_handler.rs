//@error-in-other-file: aborted
//@normalize-stderr-test: "\|.*::abort\(\).*" -> "| ABORT()"
//@normalize-stderr-test: "\| +\^+" -> "| ^"
#![feature(allocator_api)]

use std::alloc::*;

#[cfg_attr(kani, kani::proof)]
fn main() {
    handle_alloc_error(Layout::for_value(&0));
}
