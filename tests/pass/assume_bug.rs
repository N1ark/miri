//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
#[cfg_attr(kani, kani::proof)]
fn main() {
    vec![()].into_iter();
}
