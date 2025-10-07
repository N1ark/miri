//@only-target: darwin

use std::cell::UnsafeCell;

#[cfg_attr(kani, kani::proof)]
fn main() {
    let lock = UnsafeCell::new(libc::OS_UNFAIR_LOCK_INIT);

    unsafe {
        libc::os_unfair_lock_unlock(lock.get());
        //~^ error: abnormal termination: attempted to unlock an os_unfair_lock not owned by the current thread
    }
}
