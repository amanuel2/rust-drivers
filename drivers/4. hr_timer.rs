// SPDX-License-Identifier: GPL-2.0

//! Rust HR Timer.
//! https://lwn.net/Articles/167897/

use kernel::prelude::*;
use kernel::time;

/// The primary users of precision timers are user-space applications that utilize nanosleep,
/// posix-timers and itimer interfaces. Also, in-kernel users like drivers and subsystems which require
/// precise timed events (e.g. multimedia) can benefit from the availability of a separate
/// high-resolution timer subsystem as well.
///
/// Core differences
/// - Rather than using the "timer wheel" data structure, hrtimers live on a time-sorted linked list, with the next timer to expire being at the head of the list. 
/// - A separate red/black tree is also used to enable the insertion and removal of timer events without scanning through the list. 
/// - HRTimers are based on ktime_t, which is a 64-bit signed integer representing nanoseconds.
module! {
    type: RustHrtimer,
    name: "rust_hrtimer",
    author: "Rust for Linux Contributors",
    description: "Rust timer device with rw",
    license: "GPL",
}

struct RustHRTimer {
    timer: Box<time::hrtimer>
}

pub unsafe extern "C" fn hrtimer_cb(_timer: *mut time::hrtimer) -> time::hrtimer_restart {
    pr_info!("HRTimer Rust Hit!!");
    0
}

impl kernel::Module for RustHRTimer {
    fn init(_module: &'static ThisModule) -> kernel::error::Result<Self> {
        // let mut hrtimer: Box<time::hrtimer> = match Box::new(time::hrtimer {
        //     node: time::timerqueue_node {
        //         next: core::ptr::null_mut(),
        //         prev: core::ptr::null_mut(),
        //     },
        //     _softexpires: time::ktime_t { tv64: 0 },
        //     function: Some(hrtimer_cb),
        //     base: core::ptr::null_mut(),
        //     state: 0,
        //     is_rel: 0,
        //     is_soft: 0, // softirq
        //     is_hard: 0,
        // }) {
        //     Ok(hrtimer) => hrtimer,
        //     Err(_) => panic!("Could not allocate hrtimer"),
        // };
        //
        let mut timer = Box::try_new(time::hrtimer::default()).expect("Could not allocate hrtimer");

        unsafe {
            // CLOCK_MONOTONIC: a clock which is guaranteed always to move forward in time,
            // CLOCK_REALTIME: matches real world time  
            time::hrtimer_init(timer.as_mut(), time::CLOCK_MONOTONIC, time::HRTIMER_MODE_REL);
            timer.function = Some(hrtimer_cb);

            // ktime_set(seconds, nanoseconds)
            time::hrtimer_start_range_ns(timer.as_mut(), time::ktime_set(10, 0), 0, time::HRTIMER_MODE_REL);
        }

        Ok(RustHRTimer {timer})
    }
}

impl Drop for RustHRTimer {
    fn drop(&mut self) {
        unsafe { time::hrtimer_cancel(self.timer.as_mut()) };
        pr_info!("Rust hrtimer sample (exit)\n");
    }
}

