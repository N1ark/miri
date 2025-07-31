//! Even referencing an unknown `extern static` already triggers an error.

extern "C" {
    static E: [u8; 0];
}

static X: &'static [u8; 0] = unsafe { &E };

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _val = X; //~ ERROR: is not supported by Miri
}
