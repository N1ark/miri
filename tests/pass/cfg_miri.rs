#[kani::proof]
fn main() {
    assert!(cfg!(miri));
}
