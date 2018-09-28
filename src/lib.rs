#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(read_initializer)]

#![cfg(windows)]

extern crate libc;
extern crate winapi;

#[macro_use]
extern crate cfg_if;

pub mod os;
#[macro_use]
mod sys_common;
mod sys;

mod net;