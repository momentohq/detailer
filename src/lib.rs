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
//! # Example Output
//! For a query command web service api with some detail!() statements, you might have a trace log that looks like this:
//! ```text
//! 2024-04-24T20:31:28.767641975+00:00 INFO detailer::detailer - 0      detail start
//! 14     running query command for ip 10.0.0.17
//! 16     authenticating
//! 18       authorization header parsed
//! 18       identity matches request
//! 23     throttling
//! 23     authorizing
//! 23       no matching resource policy
//! 23       permitted by action policy
//! 24     executing user count query
//! 837    request complete: Ok("42")
//! 843    dropped
//! ```
//! The schema of messages is `{µs since trace start}   {message}`
//!
//! In this example there are some observations you can make:
//! * Framework code takes 14µs to get into the request handler
//! * Authentication is working well - only about 2µs for this request.
//! * Throttling and authorization are doing very well.
//! * The authorization header passed in had no resource policy, but it had an action policy.
//! * The backend took around 813µs to complete the query. It was successful.
//! * The framework is more trim on the tail end of the request, only taking 6µs to complete and drop.
//!
//! You might want to `detail!()` the backend client a little more in this example. It's masking the bulk of your wall
//! clock query request time. If 813µs is good for your backend, however, maybe this is just a good trace result.

#[deny(missing_docs)]
mod detailer;

pub use detailer::{DetailScopeGuard, Detailer, TimingSetting};
