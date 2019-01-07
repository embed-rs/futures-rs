//! Asynchronous channels.
//!
//! This crate provides channels that can be used to communicate between
//! asynchronous tasks.

#![feature(futures_api)]
#![feature(alloc)]
#![feature(asm)]

#![cfg_attr(not(feature = "std"), no_std)]

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#![doc(html_root_url = "https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures_channel")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
mod lock;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod mpsc;
#[cfg(feature = "std")]
pub mod oneshot;
