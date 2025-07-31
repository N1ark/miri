#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(std::char::from_u32('x' as u32), Some('x'));
}
