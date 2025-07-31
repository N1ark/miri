//@compile-flags: -Zmiri-disable-isolation

#[cfg_attr(kani, kani::proof)]
fn main() {
    assert_eq!(std::env::var("MIRI_ENV_VAR_TEST"), Ok("0".to_owned()));
}
