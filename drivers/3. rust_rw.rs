// allow missing docs
#![allow(missing_docs)]

use core::cmp::min;

use kernel::fs::file;
use kernel::prelude::*;

// use kernel::device;
use kernel::fs;
use kernel::uaccess::UserSlice;
use kernel::types::loff_t;
use kernel::alloc;
//use kernel::workqueue;
//use kernel::error;


module! {
    type: RustRW,
    name: "rust_rw",
    author: "Rust for Linux Contributors",
    description: "Rust chr device with rw",
    license: "GPL",
}

///// cdev binding
/// #[repr(C)]
/// #[derive(Copy, Clone)]
/// pub struct cdev {
///     pub kobj: kobject,
///     pub owner: *mut module,
///     pub ops: *const file_operations,
///     pub list: list_head,
///     pub dev: dev_t,
///     pub count: core::ffi::c_uint,
/// }

struct RustRW {
    pub buf: [u8; 255]
}

// initalize empty
static mut BUF: &[u8; 255] = &[0; 255]; // kernel buffer
static BUF_PTR: usize= 0;

/// file ops implementations
pub unsafe extern "C" fn driver_read(_file: *mut fs::file::linux_file, user_buf: *mut i8, cnt: usize, _off: *mut loff_t) -> isize {
    /*len to copy*/
    let len = min(cnt, BUF_PTR);
    let mut reader = unsafe { UserSlice::new(BUF.as_ptr() as usize, len).reader() };

    let user_slice = unsafe { core::slice::from_raw_parts_mut(user_buf as *mut u8, len) };
    let res = reader.read_slice(user_slice);


    match res{
        Ok(_) => pr_info!("yay"),
        Err(_err) => pr_alert!("Error writing! {}", _err.to_errno()),
    };

    len as isize
}

pub unsafe extern "C" fn driver_write(_file: *mut fs::file::linux_file, user_buf: *const i8, cnt: usize, _off: *mut loff_t) -> isize {
    let len = min(cnt, BUF_PTR);
    let mut writer = unsafe { UserSlice::new(BUF.as_ptr() as usize, len).writer() };

    //let writer = uaccess::UserSliceWriter {ptr: user_buf, length:len};
    let not_copied = unsafe { writer.write_slice(CStr::from_char_ptr(user_buf as *const i8).as_bytes_with_nul())};
    
    match not_copied {
        Ok(_) => pr_info!("yay"),
        Err(_err) => pr_alert!("Error reading! {}", _err.to_errno()),
    };
    // errno's if all is not copied (EFAULT)

    len as isize
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

impl kernel::Module for RustRW {
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

        let test: RustRW = RustRW { buf: [0; 255] };
        pr_info!("RustRW buf {:?} ", test.buf);
        
        let mut my_dev: Box<file::dev_t> = match alloc::box_ext::BoxExt::<file::dev_t>::new(0,GFP_KERNEL) {
            Ok(dev) => dev,
            Err(_) => panic!("Could not allocate dev_t"),
        };

        let name = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"aman\0") };
        let mut res = unsafe { file::alloc_chrdev_region(my_dev.as_mut() , 42, 69, name.as_ptr() as *const i8) };
        if res < 0 {
            pr_err!("Could not alloc");
        }
        
        // create class holder
        let my_class: *mut file::class = unsafe { file::class_create(name.as_ptr() as *const i8) };
        // create dev file
        unsafe { file::device_create(my_class as *const file::class, core::ptr::null_mut(), *my_dev, core::ptr::null_mut(), name.as_ptr() as *const i8) };

        let mut my_ch_dev: file::cdev = Default::default();
        // register dev to kernel
        res = unsafe { file::cdev_add(&mut my_ch_dev as *mut file::cdev, *my_dev, 1) };
        
        if res < 0 { pr_err!("Nooo!") }
        
        pr_info!("Device registered!!\n");
        Ok(test)
    }
}


impl Drop for RustRW {
    fn drop(&mut self) {
        pr_info!("Driver getting dropped!")
    }
}

