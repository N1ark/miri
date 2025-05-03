use std::alloc::{Layout, alloc, dealloc};

#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        let x = alloc(Layout::from_size_align_unchecked(1, 1));
        dealloc(x, Layout::from_size_align_unchecked(1, 1));
        dealloc(x, Layout::from_size_align_unchecked(1, 1)); //~ERROR: has been freed
    }
}
