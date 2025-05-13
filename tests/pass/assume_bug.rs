//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
#[kani::proof]
fn main() {
    vec![()].into_iter();
}
