#![cfg_attr(not(feature = "tests"), no_std)]
extern crate alloc;

pub mod error;
pub mod hxposed;
mod intern;
pub mod services;