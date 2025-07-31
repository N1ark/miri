use std::sync::Arc;

trait Foo {}

impl Foo for [u8; 2] {}

#[cfg_attr(kani, kani::proof)]
fn main() {
    let _val: Arc<dyn Foo + Send> = Arc::new([3, 4]);
}
