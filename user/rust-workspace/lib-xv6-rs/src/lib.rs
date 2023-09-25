#![no_std]
#![feature(lang_items)]

extern crate alloc;

use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use crate::raw_ulibc::{free, malloc};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    write(std_fd::STDERR, "Rust panic happened lol, you are shit");
    exit(1);
    loop {}
}


#[allow(dead_code)]
mod raw_syscalls {
    use core::ffi::c_void;

    #[link(name = "userxv", kind = "static")]
    extern "C" {
        pub fn fork() -> i32;
        pub fn exit(exit_code: i32) -> i32;
        pub fn wait(status: *mut i32) -> i32;
        pub fn pipe(p: *mut i32) -> i32;
        pub fn write(fd: i32, buf: *const c_void, count: i32) -> i32;
        pub fn read(fd: i32, buf: *mut c_void, count: i32) -> i32;
        pub fn close(fd: i32) -> i32;
        pub fn kill(pid: i32) -> i32;
        // pub fn exec(const char*, char**);
        // pub fn open(const char*, int);
        // pub fn mknod(const char*, short, short);
        // pub fn unlink(const char*);
        // pub fn fstat(int fd, struct stat*);
        // pub fn link(const char*, const char*);
        // pub fn mkdir(const char*);
        // pub fn chdir(const char*);
        pub fn dup(fd: i32) -> i32;
        pub fn getpid() -> i32;
        pub fn sbrk(n: i32) -> *mut u8;
        pub fn sleep(n: i32) -> i32;
        pub fn uptime() -> i32;
        pub fn dump() -> i32;
        pub fn dump2(pid: i32, register_num: i32, return_value: *mut i64) -> i32;
    }
}

mod raw_ulibc {
    use core::ffi::c_void;

    #[link(name = "userxv", kind = "static")]
    extern "C" {
        pub fn malloc(n: u32) -> *mut c_void;
        pub fn free(n: *mut c_void);
    }
}

struct Xv6MallocWrapper;

unsafe impl Sync for Xv6MallocWrapper {}

unsafe impl GlobalAlloc for Xv6MallocWrapper {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        free(ptr as *mut c_void);
    }
}

#[global_allocator]
static HEAP: Xv6MallocWrapper = Xv6MallocWrapper;

#[derive(Copy, Clone)]
pub struct FileDescriptor {
    fd: i32,
}

pub mod std_fd {
    use crate::FileDescriptor;

    pub static STDIN: FileDescriptor = FileDescriptor { fd: 0 };
    pub static STDOUT: FileDescriptor = FileDescriptor { fd: 1 };
    pub static STDERR: FileDescriptor = FileDescriptor { fd: 2 };
}


pub fn exit(code: i32) {
    unsafe {
        raw_syscalls::exit(code);
    }
}

pub fn write(fd: FileDescriptor, input: &str) {
    unsafe {
        raw_syscalls::write(fd.fd, input.as_bytes().as_ptr() as *const c_void, input.as_bytes().len() as i32);
    }
}