#![feature(asm)]
#![feature(box_syntax)]
#![feature(core_intrinsics)]
#![feature(fnbox)]
#![feature(min_const_fn)]
#![feature(read_initializer)]

#![cfg(windows)]

extern crate libc;
extern crate winapi;

#[cfg(test)]
extern crate rand;

#[macro_use]
extern crate cfg_if;

pub mod os;
#[macro_use]
mod sys_common;
mod sys;

mod net;