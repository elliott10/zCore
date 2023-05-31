//! Zircon kernel objects
//!
//! # Feature flags
//!
//! - `elf`: Enables `zircon_object::util::elf_loader`.
//! - `hypervisor`: Enables `zircon_object::hypervisor` (`Guest` and `Vcpu`).

#![no_std]
#![deny(unused_must_use, missing_docs)]
//#![deny(warnings)]
//#![deny(unsafe_code)]
#![feature(drain_filter)]
#![feature(get_mut_unchecked)]

extern crate alloc;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod debuglog;
pub mod dev;
mod error;
#[cfg(feature = "hypervisor")]
pub mod hypervisor;
pub mod ipc;
pub mod object;
pub mod signal;
pub mod task;
pub mod util;
pub mod vm;

pub use self::error::*;
