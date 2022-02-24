pub use crate::{
    error::{TryFromIntError, TryMergeError, TryMergeErrorKind},
    hyperloglog::HyperLogLog,
    precision::Precision,
};

mod error;
mod hyperloglog;
mod precision;
mod registers;
