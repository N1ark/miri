#![feature(custom_mir, core_intrinsics)]
use std::intrinsics::mir::*;

pub struct S(i32);

#[custom_mir(dialect = "runtime", phase = "optimized")]
#[cfg_attr(kani, kani::proof)]
fn main() {
    mir! {
        let _unit: ();
        let _observe: i32;
        {
            let non_copy = S(42);
            // This could change `non_copy` in-place
            Call(_unit = change_arg(Move(non_copy)), ReturnTo(after_call), UnwindContinue())
        }
        after_call = {
            // So now we must not be allowed to observe non-copy again.
            _observe = non_copy.0; //~ERROR: uninitialized
            Return()
        }

    }
}

pub fn change_arg(mut x: S) {
    x.0 = 0;
}
