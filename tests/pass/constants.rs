const A: usize = *&5;

fn foo() -> usize {
    A
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(foo(), A);
}
