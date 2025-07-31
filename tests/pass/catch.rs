use std::panic::{AssertUnwindSafe, catch_unwind};

#[cfg_attr(kani, kani::proof)]
fn main() {
    let mut i = 3;
    let _val = catch_unwind(AssertUnwindSafe(|| i -= 2));
    println!("{}", i);
}
