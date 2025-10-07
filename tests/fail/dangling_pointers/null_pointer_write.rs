#[allow(deref_nullptr)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe { *std::ptr::null_mut() = 0i32 }; //~ ERROR: null pointer
}
