#[cfg_attr(kani, kani::proof)]
fn main() {
    assert!(Ok::<i32, String>(42) == Ok(42));
}
