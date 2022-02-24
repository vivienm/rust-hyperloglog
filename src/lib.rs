//! A [hyperloglog](https://en.wikipedia.org/wiki/HyperLogLog) implementation in Rust.
//!
//! ```
//! use hyperloglog::HyperLogLog;
//!
//! let mut hll = HyperLogLog::<i32>::new();
//! for i in 0..100_000 {
//!     hll.insert(&i);
//! }
//! assert!((50_000..150_000).contains(&hll.len()));
//! ```
pub use crate::{
    error::{TryFromIntError, TryMergeError, TryMergeErrorKind},
    hyperloglog::HyperLogLog,
    precision::Precision,
};

mod error;
mod hyperloglog;
mod precision;
mod registers;
