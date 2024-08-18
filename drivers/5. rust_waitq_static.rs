
// allow missing docs
#![allow(missing_docs)]

use core::cmp::min;

use kernel::fs::file;
use kernel::task;
use kernel::prelude::*;
use kernel::sync::CondVar;
use kernel::sync;
use kernel::sync::lock::Lock;
// use kernel::device;
use kernel::fs;
use kernel::types::loff_t;
//use kernel::workqueue;
//use kernel::error;
use kernel::device::unreg_chrdev;

module! {
    type: RustWaitQStatic,
    name: "rust_rw",
    author: "Rust for Linux Contributors",
    description: "Rust chr device with rw",
    license: "GPL",
}
struct RustWaitQStatic {
    pub buf: [u8; 255],
    pub cdev: file::cdev,
}

pub unsafe extern "C" fn driver_read(
    _file: *mut fs::file::linux_file,
    user_buf: *mut i8,
    cnt: usize,
    _off: *mut loff_t,
) -> isize {
}

pub unsafe extern "C" fn driver_write(
    _file: *mut fs::file::linux_file,
    user_buf: *const i8,
    cnt: usize,
    _off: *mut loff_t,
) -> isize {
}

#[allow(missing_docs)]
pub unsafe extern "C" fn driver_open(
    _device_flie: *mut fs::file::inode,
    _instance: *mut fs::file::linux_file,
) -> i32 {
    0
}

#[allow(missing_docs)]
pub unsafe extern "C" fn driver_close(
    _device_flie: *mut fs::file::inode,
    _instance: *mut fs::file::linux_file,
) -> i32 {
    0
}

pub unsafe extern "C" fn driver_waiter(_data: *mut core::ffi::c_void) -> i32 {
    loop {
        pr_info!("Waiting for event....");
        // create a Cstr with static lifetime

        // Create a static condition variable
        static MY_COND: CondVar = CondVar::new("WaitQStatic");

        // Create a static lock
        static MY_LOCK: Lock= Lock::new("WaitQStatic", sync::LockClassKey::new());

        // acquire guard for lock
        let guard = MY_LOCK.lock();

        
        match MY_COND.wait_interruptible_timeout(&guard, 1000) {
            Ok(0) => pr_info!("Timeout occurred"),
            Ok(_) => pr_info!("Woken up before timeout"),
            Err(e) => pr_err!("Error while waiting: {:?}", e),
        }
    }

    // do_exit() to call schedule() to switch to another task
    // then deallocate to prevent orphan
}

impl kernel::Module for RustWaitQStatic {
    fn init(_module: &'static ThisModule) -> kernel::error::Result<Self> {
        let _ops = fs::file::file_ops {
            owner: core::ptr::null_mut(), //fs::file::this_module.as_ptr(),
            llseek: None,
            fop_flags: 0,
            read: Some(driver_read),
            write: Some(driver_write),
            read_iter: None,
            write_iter: None,
            iopoll: None,
            iterate_shared: None,
            poll: None,
            unlocked_ioctl: None,
            compat_ioctl: None,
            mmap: None,
            open: Some(driver_open),
            flush: None,
            release: Some(driver_close),
            fsync: None,
            fasync: None,
            lock: None,
            get_unmapped_area: None,
            check_flags: None,
            flock: None,
            splice_write: None,
            splice_read: None,
            splice_eof: None,
            setlease: None,
            fallocate: None,
            show_fdinfo: None,
            copy_file_range: None,
            remap_file_range: None,
            fadvise: None,
            uring_cmd: None,
            uring_cmd_iopoll: None,
        };

        pr_info!("Registering device\n");

        let mut my_ch_dev: file::cdev = Default::default();
        let res = unsafe { file::cdev_add(&mut my_ch_dev as *mut file::cdev, *my_ch_dev, 1) }; // register dev to kernel

        if res < 0 {
            panic!("Couldn't create cdev")
        }

        let name = b"rust_wait_q_s\0";
        // create class holder
        let my_class: *mut file::class = unsafe { file::class_create(name.as_ptr() as *const i8) };
        // create dev file
        unsafe {
            file::device_create(
                my_class as *const file::class,
                core::ptr::null_mut(),
                *my_ch_dev,
                core::ptr::null_mut(),
                name.as_ptr() as *const i8,
            )
        };

        // kthread create
        let my_task = task::kthread_create_on_node(
            Some(driver_waiter),
            core::ptr::null_mut(),
            -1, // NUMA_NO_NODe
            name.as_ptr() as *const i8, 
        );

    
        pr_info!("Rust Static WaitQ registered!!\n");
        Ok(RustWaitQStatic {
            buf: [0; 255],
            cdev: my_ch_dev,
        })
    }
}

impl Drop for RustWaitQStatic {
    fn drop(&mut self) {
        let name = b"rust_wait_q_s\0";
        let s = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(name) };
        unsafe { file::unreg_chrdev(42, 69, s.as_ptr() as *const i8) };
        pr_info!("Rust Static WaitQ exit\n");
    }
}
