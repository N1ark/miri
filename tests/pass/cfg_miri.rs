#[cfg_attr(kani, kani::proof)]
fn main() {
    assert!(cfg!(miri));
}
