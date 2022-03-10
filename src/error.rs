use std::{
    error::Error,
    fmt::{self, Display},
};

/// Error type returned when converting an integer into a [`Precision`](crate::Precision) fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromIntError(pub(crate) ());

impl Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "out of bounds".fmt(f)
    }
}

impl Error for TryFromIntError {}

/// A list specifying categories of merging error.
///
/// It is used with the [`TryMergeError`] type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TryMergeErrorKind {
    /// Hyperloglogs have different precisions.
    Precision,
    /// Hyperloglogs use different hashers.
    Hasher,
}

/// Error type returned when a merge operation fails.
///
/// # Examples
///
/// ```
/// use hyperloglog::{HyperLogLog, Precision, TryMergeError, TryMergeErrorKind};
///
/// let hll1 = HyperLogLog::<i32>::with_precision(Precision::P10);
/// let hll2 = HyperLogLog::<i32>::with_precision(Precision::P12);
/// let error = hll1.try_merge(&hll2).unwrap_err();
/// assert_eq!(error.kind(), TryMergeErrorKind::Precision);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryMergeError {
    pub(crate) kind: TryMergeErrorKind,
}

impl TryMergeError {
    /// Returns the corresponding [`TryMergeErrorKind`] for this error.
    #[inline]
    pub fn kind(&self) -> TryMergeErrorKind {
        self.kind
    }
}

impl Display for TryMergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(
            match self.kind() {
                TryMergeErrorKind::Precision => "incompatible precisions",
                TryMergeErrorKind::Hasher => "incompatible hashers",
            },
            f,
        )
    }
}

impl Error for TryMergeError {}
