#[cfg_attr(kani, kani::proof)]
fn main() {
    let x = "hello";
    match x {
        "foo" => {}
        "bar" => {}
        _ => {}
    }
}
