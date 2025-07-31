#[allow(dead_code)]
enum Two {
    A,
    B,
}
impl Drop for Two {
    fn drop(&mut self) {}
}
#[cfg_attr(kani, kani::proof)]
fn main() {
    let _k = Two::A;
}
