#[cfg_attr(kani, kani::proof)]
fn main() {
    let f: fn(i32) -> Option<i32> = Some::<i32>;
    f(42);
}
