#[allow(deref_nullptr)]
#[cfg_attr(kani, kani::proof)]
fn main() {
    let x: i32 = unsafe { *std::ptr::null() }; //~ ERROR: null pointer
    panic!("this should never print: {}", x);
}
