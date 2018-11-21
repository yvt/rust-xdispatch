//! This crate reexports items from [`dispatch`]. TODO: In order to make it
//! available to other platforms than macOS/iOS, it links `xdispatch-core`,
//! which includes a statically linked version of [XDispatch], a cross-platform
//! port of Apple Grand Central Dispatch, and defines stub symbols for
//! features unsupported by XDispatch.
//!
//! [`dispatch`]: https://crates.io/crates/dispatch
//! [XDispatch]: http://opensource.mlba-team.de/xdispatch/
//!
//! # Examples
//!
//! Below are some examples from `dispatch`:
//!
//! ```
//! use xdispatch::{Queue, QueueAttribute};
//!
//! let queue = Queue::create("com.example.rust", QueueAttribute::Serial);
//! queue.async(|| println!("Hello"));
//! queue.async(|| println!("World"));
//! ```
//!
//! ```
//! use xdispatch::{Queue, QueuePriority};
//!
//! let queue = Queue::global(QueuePriority::Default);
//!
//! let mut nums = vec![1, 2];
//! queue.foreach(&mut nums, |x| *x += 1);
//! assert!(nums == [2, 3]);
//!
//! let nums = queue.map(nums, |x| x.to_string());
//! assert!(nums[0] == "2");
//! ```
//!
//! <!--
//!     The true purpose of including doctests here is to make sure `xdispatch`
//!     works when linked as a part of dylib output. As opposed to normal tests,
//!     doctests are compiled in this way.
//! -->
extern crate dispatch;

#[doc(no_inline)]
pub use dispatch::*;
