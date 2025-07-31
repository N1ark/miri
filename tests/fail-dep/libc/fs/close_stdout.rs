//@ignore-target: windows # No libc IO on Windows
//@compile-flags: -Zmiri-disable-isolation

// FIXME: standard handles cannot be closed (https://github.com/rust-lang/rust/issues/40032)

#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        libc::close(1); //~ ERROR: cannot close stdout
    }
}
