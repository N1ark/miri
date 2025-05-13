//@compile-flags: -Zmiri-disable-isolation

#[kani::proof]
fn main() {
    assert_eq!(num_cpus::get(), 1);
}
