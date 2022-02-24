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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of() {
        use std::mem::size_of;
        assert_eq!(size_of::<Precision>(), size_of::<u8>());
        assert_eq!(size_of::<Option<Precision>>(), size_of::<u8>());
    }

    #[test]
    fn min_max() {
        assert_eq!(Precision::MIN.get(), 4);
        assert_eq!(Precision::MAX.get(), 18);
    }

    #[test]
    fn in_range() {
        assert!(!Precision::in_range(0));
        assert!(Precision::in_range(4));
        assert!(Precision::in_range(12));
        assert!(Precision::in_range(18));
        assert!(!Precision::in_range(20));
    }

    #[test]
    fn new() {
        assert_eq!(Precision::new(0), None);
        assert_eq!(Precision::new(4), Some(Precision::P4));
        assert_eq!(Precision::new(12), Some(Precision::P12));
        assert_eq!(Precision::new(18), Some(Precision::P18));
        assert_eq!(Precision::new(20), None);
    }

    #[test]
    fn into_int() {
        assert_eq!(Into::<u8>::into(Precision::P12), 12);
    }

    #[test]
    fn try_from_int() {
        assert_eq!(TryInto::<Precision>::try_into(0), Err(TryFromIntError(())));
        assert_eq!(TryInto::<Precision>::try_into(4), Ok(Precision::P4));
        assert_eq!(TryInto::<Precision>::try_into(12), Ok(Precision::P12));
        assert_eq!(TryInto::<Precision>::try_into(18), Ok(Precision::P18));
        assert_eq!(TryInto::<Precision>::try_into(20), Err(TryFromIntError(())));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Precision::P12), "12");
    }
}
