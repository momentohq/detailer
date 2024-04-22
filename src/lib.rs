//! A dynamic, simple workflow trace logger.
//!
//! [`Detailer`] is a log recording tool, depending only on std and log.
//! It prioritizes ease of use and low overhead, particularly when disabled.
//!
//! # Examples
//! ```rust
//! use detailer::{detail, new_detailer};
//!
//! let mut detailer = new_detailer!(); // Info level, WithTimings
//! detail!(detailer, "some {} message", "log");
//! ```
//!
//! ```rust
//! use detailer::{detail_at, new_detailer};
//!
//! let mut detailer = new_detailer!(Off); // Disabled
//! detail_at!(detailer, Error, "this is {} logged", "NOT");
//! ```
//!

#[deny(missing_docs)]
mod detailer;

pub use detailer::{Detailer, TimingSetting};
