struct T(&'static [isize]);
static STATIC: T = T(&[5, 4, 3]);
#[cfg_attr(kani, kani::proof)]
pub fn main() {
    let T(ref v) = STATIC;
    assert_eq!(v[0], 5);
}
