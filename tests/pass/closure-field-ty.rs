// miri issue #304
#[cfg_attr(kani, kani::proof)]
fn main() {
    let mut y = 0;
    {
        let mut box_maybe_closure = Box::new(None);
        *box_maybe_closure = Some(|| y += 1);
        (box_maybe_closure.unwrap())();
    }
    assert_eq!(y, 1);
}
