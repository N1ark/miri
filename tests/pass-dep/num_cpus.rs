//@compile-flags: -Zmiri-disable-isolation

#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(num_cpus::get(), 1);
}
