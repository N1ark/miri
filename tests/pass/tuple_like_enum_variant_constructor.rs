#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(Some(42).map(Some), Some(Some(42)));
}
