//! `xdispatch-core` provides a statically linked version of [XDispatch],
//! and its dependencies including [libpthread_workqueue] and [libkqueue].
//!
//! [XDispatch]: http://opensource.mlba-team.de/xdispatch/
//! [libpthread_workqueue]: http://mark.heily.com/libpthreadworkqueue/
//! [libkqueue]: http://mark.heily.com/project/libkqueue/
//!
//! This crate provides no symbols for macOS/iOS targets where the operation
//! system already provides the system version of Grand Central Dispatch.
//!
//! # Differences from the original XDispatch
//!
//!  - Does not make use of CMake.
//!  - TODO: Some parts of the platform abstraction were replaced with
//!    platform-indepependent code written in Rust.
//!