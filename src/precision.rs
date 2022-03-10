use std::fmt::{self, Display};

use crate::error::TryFromIntError;

/// The precision of the hyperloglog, that is,
/// the number of bytes used for sharding accross registers.
///
/// # Examples
///
/// ```
/// use hyperloglog::{HyperLogLog, Precision};
///
/// let hll = HyperLogLog::<i32>::with_precision(Precision::P12);
/// ```
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Precision {
    /// 4-bit precision.
    P4 = 4,
    /// 5-bit precision.
    P5 = 5,
    /// 6-bit precision.
    P6 = 6,
    /// 7-bit precision.
    P7 = 7,
    /// 8-bit precision.
    P8 = 8,
    /// 9-bit precision.
    P9 = 9,
    /// 10-bit precision.
    P10 = 10,
    /// 11-bit precision.
    P11 = 11,
    /// 12-bit precision (default).
    P12 = 12,
    /// 13-bit precision.
    P13 = 13,
    /// 14-bit precision.
    P14 = 14,
    /// 15-bit precision.
    P15 = 15,
    /// 16-bit precision.
    P16 = 16,
    /// 17-bit precision.
    P17 = 17,
    /// 18-bit precision.
    P18 = 18,
}

impl Precision {
    /// The smallest precision: 4 bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::Precision;
    ///
    /// assert_eq!(Precision::MIN, Precision::P4);
    /// ```
    pub const MIN: Self = Self::P4;

    /// The largest precision: 18 bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::Precision;
    ///
    /// assert_eq!(Precision::MAX, Precision::P18);
    /// ```
    pub const MAX: Self = Self::P18;

    /// All possible precisions, in increasing order.
    pub const fn variants<'a>() -> &'a [Self] {
        &[
            Self::P4,
            Self::P5,
            Self::P6,
            Self::P7,
            Self::P8,
            Self::P9,
            Self::P10,
            Self::P11,
            Self::P12,
            Self::P13,
            Self::P14,
            Self::P15,
            Self::P17,
            Self::P18,
        ]
    }

    /// Checks whether the given value is in the range of valid precision values.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::Precision;
    ///
    /// assert!(Precision::in_range(12));
    /// ```
    pub const fn in_range(value: u8) -> bool {
        Self::MIN.get() <= value && value <= Self::MAX.get()
    }

    /// Creates a precision if the given value is in range.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::Precision;
    ///
    /// assert_eq!(Precision::new(12), Some(Precision::P12));
    /// ```
    pub const fn new(value: u8) -> Option<Self> {
        match value {
            4 => Some(Self::P4),
            5 => Some(Self::P5),
            6 => Some(Self::P6),
            7 => Some(Self::P7),
            8 => Some(Self::P8),
            9 => Some(Self::P9),
            10 => Some(Self::P10),
            11 => Some(Self::P11),
            12 => Some(Self::P12),
            13 => Some(Self::P13),
            14 => Some(Self::P14),
            15 => Some(Self::P15),
            16 => Some(Self::P16),
            17 => Some(Self::P17),
            18 => Some(Self::P18),
            _ => None,
        }
    }

    /// Return the precision value as an [`u8`].
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::Precision;
    ///
    /// assert_eq!(Precision::P12.get(), 12);
    /// ```
    #[inline]
    pub const fn get(self) -> u8 {
        self as u8
    }
}

impl From<Precision> for u8 {
    #[inline]
    fn from(precision: Precision) -> Self {
        precision.get()
    }
}

impl TryFrom<u8> for Precision {
    type Error = TryFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(TryFromIntError(()))
    }
}

impl Default for Precision {
    fn default() -> Self {
        Self::P12
    }
}

impl Display for Precision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
