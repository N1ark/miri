extern "Rust" {
    fn miri_backtrace_size(flags: u64) -> usize;
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    unsafe {
        miri_backtrace_size(2); //~ ERROR:  unsupported operation: unknown `miri_backtrace_size` flags 2
    }
}
