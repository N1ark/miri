#[cfg_attr(kani, kani::proof)]
fn main() {
    let _val = ::std::str::from_utf8(b"a");
}
