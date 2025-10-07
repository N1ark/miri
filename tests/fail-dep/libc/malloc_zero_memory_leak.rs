#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        let _ptr = libc::malloc(0); //~ERROR: memory leak
    }
}
