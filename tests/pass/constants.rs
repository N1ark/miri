const A: usize = *&5;

fn foo() -> usize {
    A
}

#[kani::proof]
fn main() {
    assert_eq!(foo(), A);
}
