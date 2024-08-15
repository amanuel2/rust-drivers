// allow missing docs
#![allow(missing_docs)]

use kernel::prelude::BoxExt;
use kernel::prelude::*;
use kernel::time;

module! {
    type: RustTimer,
    name: "rust_timer",
    author: "Rust for Linux Contributors",
    description: "Rust chr device with rw",
    license: "GPL",
}

struct RustTimer {}

pub unsafe extern "C" fn timer_cb(_timer: *mut time::timer_list) {
    pr_info!("Timer Rust Hit!!");
}

impl kernel::Module for RustTimer {
    fn init(_module: &'static ThisModule) -> kernel::error::Result<Self> {
        let timer_inner: time::timer_list = time::timer_list {
            entry: time::hlist_node {
                next: core::ptr::null_mut(),
                pprev: core::ptr::null_mut(),
            },
            expires: 0,
            function: None,
            flags: 0,
        };

        let mut timer: Box<time::timer_list> = match Box::new(timer_inner, kernel::alloc::flags::GFP_KERNEL) {
            Ok(timer) => timer,
            Err(_) => panic!("Could not allocate timer"),
        };

        unsafe {
            time::init_timer_key(
                timer.as_mut(),
                Some(timer_cb),
                0,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );

            time::mod_timer(timer.as_mut(), time::msecs_to_jiffies(10000));
        };

        Ok(RustTimer {})
    }
}

impl Drop for RustTimer {
    fn drop(&mut self) {
        pr_info!("RustTimer module destroyed");
    }
}
