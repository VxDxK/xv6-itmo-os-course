#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use lib_xv6_rs::*;
use std_fd::STDOUT;

//TODO: Make macro sl #[entry] to wrap argc/argv and maybe exit
#[no_mangle]
pub extern "C" fn main(argc: i32, _: *const *const u8) {
    let str = "Hello my Cargo friend\n";
    let str_owned = format!("This is owning string and format argc: {}\n", argc);

    write(STDOUT, str);
    write(STDOUT, str_owned.as_str());

    exit(0);
}