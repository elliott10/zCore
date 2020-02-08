//! Linux kernel objects

#![no_std]
#![deny(warnings, unsafe_code, unused_must_use, unreachable_patterns)]
#![feature(bool_to_option)]
#![feature(untagged_unions)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

// layer 0
pub mod error;
#[macro_use]
mod util;

// layer 1
pub mod fs;
pub mod net;

// layer 2
pub mod loader;
pub mod process;
pub mod thread;
