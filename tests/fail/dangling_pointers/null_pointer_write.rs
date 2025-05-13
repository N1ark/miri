#[allow(deref_nullptr)]
#[kani::proof]
fn main() {
    unsafe { *std::ptr::null_mut() = 0i32 }; //~ ERROR: null pointer
}
