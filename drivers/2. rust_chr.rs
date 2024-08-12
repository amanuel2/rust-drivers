// SPDX-License-Identifier: GPL-2.0

//! Rust character device minimal

//use core::ptr;


use kernel::device;
use kernel::fs;
use kernel::prelude::*;

#[cfg(CONFIG_MODULES)]
module! {
    type: RustChr,
    name: "rust_chr",
    author: "Aman",
    description: "Rust Simple Device",
    license: "GPL",
}

static DEV_MAJOR: u32 = 35;

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

struct RustChr {}

impl kernel::Module for RustChr {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let _ops = fs::file::file_ops {
            owner: core::ptr::null_mut(), //fs::file::this_module.as_ptr(),
            llseek: None,
            fop_flags: 0,
            read: None,
            write: None,
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

        let name = b"rust_char_dev\0";
        let s = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(name) };
        let rc = unsafe {
            device::reg_chrdev(
                DEV_MAJOR,
                0,
                1,
                s.as_ptr() as *const i8,
                &_ops as *const fs::file::file_ops,
            )
        };
        match rc {
            0 => pr_info!("succesffuly registered chr dev yayy =)\n"),
            _ => pr_err!("Something went wrong =( {}\n", rc),
        }

        Ok(RustChr {})
    }
}

impl Drop for RustChr {
    fn drop(&mut self) {
        let name = b"rust_char_dev\0";
        let s = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(name) };
        unsafe { device::unreg_chrdev(DEV_MAJOR, 0, 1, s.as_ptr() as *const i8) };
        pr_info!("Rust chr_dev exit\n");
    }
}
