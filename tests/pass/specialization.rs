#![allow(incomplete_features)]
#![feature(specialization)]

trait IsUnit {
    fn is_unit() -> bool;
}

impl<T> IsUnit for T {
    default fn is_unit() -> bool {
        false
    }
}

impl IsUnit for () {
    fn is_unit() -> bool {
        true
    }
}

fn specialization() -> (bool, bool) {
    (i32::is_unit(), <()>::is_unit())
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(specialization(), (false, true));
}
