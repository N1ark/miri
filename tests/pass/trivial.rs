fn empty() {}

fn unit_var() {
    let x = ();
    x
}

#[cfg_attr(kani, kani::proof)]
fn main() {
    empty();
    unit_var();
}
