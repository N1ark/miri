fn empty() {}

fn unit_var() {
    let x = ();
    x
}

#[kani::proof]
fn main() {
    empty();
    unit_var();
}
