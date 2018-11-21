//! # xdispatch for Rust
//!
//! This crate reexports items from [`dispatch`]. TODO: In order to make it
//! available to other platforms than macOS/iOS, it links `xdispatch-core`,
//! which includes a statically linked version of [XDispatch], a cross-platform
//! port of Apple Grand Central Dispatch, and defines stub symbols for
//! features unsupported by XDispatch.
//!
//! [`dispatch`]: https://crates.io/crates/dispatch
//! [XDispatch]: http://opensource.mlba-team.de/xdispatch/
//!
extern crate dispatch;

#[doc(no_inline)]
pub use dispatch::*;
