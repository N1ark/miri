// This once failed with "unwinding past a stack frame that does not allow unwinding",
// fixed by https://github.com/rust-lang/rust/issues/110233.

#[cfg_attr(kani, kani::proof)]
fn main() {
    let x = [1, 2, 3, 4];
    let _val = &x[..=4];
}
