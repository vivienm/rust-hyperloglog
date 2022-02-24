use std::{
    borrow::Borrow,
    collections::hash_map::{DefaultHasher, RandomState},
    fmt::{self, Debug},
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    error::{TryMergeError, TryMergeErrorKind},
    precision::Precision,
    registers::Registers,
};

fn alpha(num_bits: u8, num_registers: usize) -> f64 {
    if num_bits == 4 {
        0.673
    } else if num_bits == 5 {
        0.697
    } else if num_bits == 6 {
        0.709
    } else {
        0.7213 / (1. + 1.079 / (num_registers as f64))
    }
}

/// A hyperloglog data structure to estimate the number of distinct elements in a data stream.
///
/// ```
/// use hyperloglog::HyperLogLog;
///
/// let mut hll = HyperLogLog::<i32>::new();
/// for i in 0..100_000 {
///      hll.insert(&i);
/// }
/// assert!((50_000..150_000).contains(&hll.len()));
/// ```
pub struct HyperLogLog<T, S = BuildHasherDefault<DefaultHasher>>
where
    T: ?Sized,
{
    alpha: f64,
    registers: Registers,
    hash_builder: S,
    phantom: PhantomData<T>,
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
    S: Default,
{
    /// Creates a new empty hyperloglog with the default precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let hll = HyperLogLog::<i32>::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty hyperloglog with the given precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::{Precision, HyperLogLog};
    ///
    /// let hll = HyperLogLog::<i32>::with_precision(Precision::P12);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_precision(precision: Precision) -> Self {
        Self::with_precision_and_hasher(precision, S::default())
    }
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
{
    /// Creates a new empty hyperloglog with the default precision and the given hasher
    /// to hash keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    ///
    /// use hyperloglog::HyperLogLog;
    ///
    /// let s = RandomState::new();
    /// let mut hll = HyperLogLog::<i32, _>::with_hasher(s);
    /// hll.insert(&1);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_precision_and_hasher(Precision::default(), hasher)
    }

    /// Creates a new empty hyperloglog with the given precision and the given hasher to hash keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    ///
    /// use hyperloglog::{Precision, HyperLogLog};
    ///
    /// let s = RandomState::new();
    /// let mut hll = HyperLogLog::<i32, _>::with_precision_and_hasher(Precision::P12, s);
    /// hll.insert(&1);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_precision_and_hasher(precision: Precision, hasher: S) -> Self {
        Self::with_registers_and_hasher(Registers::with_precision(precision), hasher)
    }

    #[inline]
    fn with_registers_and_hasher(registers: Registers, hasher: S) -> Self {
        Self {
            alpha: alpha(registers.precision().get(), registers.len()),
            registers,
            hash_builder: hasher,
            phantom: PhantomData,
        }
    }

    /// Returns the precision of the hyperloglog.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::{HyperLogLog, Precision};
    ///
    /// let mut hll = HyperLogLog::<i32>::with_precision(Precision::P12);
    /// assert_eq!(hll.precision(), Precision::P12);
    /// ```
    #[inline]
    pub fn precision(&self) -> Precision {
        self.registers.precision()
    }

    /// Returns a reference to the hyperloglog's [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    ///
    /// use hyperloglog::HyperLogLog;
    ///
    /// let hasher = RandomState::new();
    /// let hll = HyperLogLog::<i32, _>::with_hasher(hasher);
    /// let hasher: &RandomState = hll.hasher();
    /// ```
    #[inline]
    pub fn hasher(&self) -> &S {
        &self.hash_builder
    }

    /// Adds a hash value to the hyperloglog.
    ///
    /// This may be handy when the hash is previously computed, to avoid computing twice.
    /// Hash values need to be uniformly distributed over [u64] for an accurate total count.
    ///
    /// In all other cases, use [`HyperLogLog::insert`] instead.
    ///
    /// ```
    /// use std::{
    ///     collections::hash_map::RandomState,
    ///     hash::{BuildHasher, Hash, Hasher},
    /// };
    ///
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hasher = RandomState::new().build_hasher();
    /// 1i32.hash(&mut hasher);
    /// let h = hasher.finish();
    ///
    /// let mut hll = HyperLogLog::<i32, ()>::with_hasher(());
    /// hll.insert_hash(h);
    /// ```
    pub fn insert_hash(&mut self, h: u64) {
        let num_bits = self.precision().get();
        // Split h into (w, j) where w represents the (64 - p) upper bits, and j the p lower bits.
        let w = h >> num_bits;
        let j = h - (w << num_bits);
        // Leftmost bit (1-based count).
        let rho = (w.leading_zeros() + 1 - (num_bits as u32)) as u8;
        self.registers.update(j as usize, rho);
    }

    /// Calculates the approximate number of different elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll = HyperLogLog::<i32>::new();
    /// for i in 0..100_000 {
    ///     hll.insert(&i);
    /// }
    /// assert!((50_000..150_000).contains(&hll.len()));
    /// ```
    pub fn len(&self) -> usize {
        let m = self.registers.len() as f64;
        let z = 1.
            / self
                .registers
                .iter()
                .map(|value| 2f64.powi(-(value as i32)))
                .sum::<f64>();
        let mut e = self.alpha * m * m * z;
        if e < 2.5 * m {
            // Small range correction.
            let v = self.registers.iter().filter(|value| *value != 0).count();
            if v != 0 {
                e = m * (m / (v as f64)).ln();
            }
        }
        // We're using 64-bit hashes, so large range correction is not needed.
        e as usize
    }

    /// Clears the hyperloglog, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll = HyperLogLog::<i32>::new();
    /// hll.insert(&1);
    /// hll.clear();
    /// assert!(hll.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.registers.clear()
    }

    /// Returns `true` if the hyperloglog contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll = HyperLogLog::<i32>::new();
    /// assert!(hll.is_empty());
    /// hll.insert(&1);
    /// assert!(!hll.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.registers.is_empty()
    }

    /// Merges the hyperloglog `rhs` into `self` without checking that precisions and hashers
    /// are the same on both terms.
    ///
    /// A checked variant is available via the [`try_merge_from`](HyperLogLog::try_merge_from)
    /// method.
    #[inline]
    pub fn merge_from_unchecked(&mut self, rhs: &Self) {
        self.registers.merge_from_unchecked(&rhs.registers);
    }
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
    S: Clone,
{
    /// Merges two hyperloglogs without checking that precisions and hashers
    /// are the same on both terms.
    ///
    /// A checked variant is available via the [`try_merge`](HyperLogLog::try_merge) method.
    pub fn merge_unchecked(&self, rhs: &Self) -> Self {
        Self::with_registers_and_hasher(
            self.registers.merge_unchecked(&rhs.registers),
            self.hash_builder.clone(),
        )
    }
}

fn check_merge_conds<T, S>(
    lhs: &HyperLogLog<T, S>,
    rhs: &HyperLogLog<T, S>,
) -> Result<(), TryMergeError>
where
    T: ?Sized,
    S: Eq,
{
    if lhs.precision() != rhs.precision() {
        return Err(TryMergeError(TryMergeErrorKind::Precision));
    }
    if lhs.hasher() != rhs.hasher() {
        return Err(TryMergeError(TryMergeErrorKind::Hasher));
    }
    Ok(())
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
    S: Eq,
{
    /// Merges the hyperloglog `rhs` into `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll1 = HyperLogLog::<i32>::new();
    /// for i in 0..75_000 {
    ///     hll1.insert(&i);
    /// }
    /// let mut hll2 = HyperLogLog::<i32>::new();
    /// for i in 25_000..100_000 {
    ///     hll2.insert(&i);
    /// }
    /// hll1.try_merge_from(&hll2).unwrap();
    /// assert!((50_000..150_000).contains(&hll1.len()));
    /// ```
    pub fn try_merge_from(&mut self, rhs: &Self) -> Result<(), TryMergeError> {
        check_merge_conds(self, rhs)?;
        self.merge_from_unchecked(rhs);
        Ok(())
    }
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
    S: Clone + Eq,
{
    /// Merges two hyperloglogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll1 = HyperLogLog::<i32>::new();
    /// for i in 0..75_000 {
    ///     hll1.insert(&i);
    /// }
    /// let mut hll2 = HyperLogLog::<i32>::new();
    /// for i in 25_000..100_000 {
    ///     hll2.insert(&i);
    /// }
    /// let hll3 = hll1.try_merge(&hll2).unwrap();
    /// assert!((50_000..150_000).contains(&hll3.len()));
    /// ```
    pub fn try_merge(&self, rhs: &Self) -> Result<Self, TryMergeError> {
        check_merge_conds(self, rhs)?;
        Ok(self.merge_unchecked(rhs))
    }
}

impl<T, S> HyperLogLog<T, S>
where
    T: ?Sized,
    S: BuildHasher,
{
    /// Adds a value to the hyperloglog.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperloglog::HyperLogLog;
    ///
    /// let mut hll = HyperLogLog::<i32>::new();
    /// hll.insert(&1);
    /// ```
    pub fn insert<Q>(&mut self, value: &Q)
    where
        T: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = self.hash_builder.build_hasher();
        value.hash(&mut hasher);
        let h = hasher.finish();
        self.insert_hash(h);
    }
}

impl<T, S> Clone for HyperLogLog<T, S>
where
    T: ?Sized,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            registers: self.registers.clone(),
            hash_builder: self.hash_builder.clone(),
            alpha: self.alpha,
            phantom: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.registers = source.registers.clone();
        self.hash_builder = source.hash_builder.clone();
    }
}

impl<T, S> Debug for HyperLogLog<T, S>
where
    T: ?Sized,
    S: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperLogLog")
            .field("alpha", &self.alpha)
            .field("registers", &self.registers)
            .field("hash_builder", &self.hash_builder)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<T, S> Default for HyperLogLog<T, S>
where
    T: ?Sized,
    S: Default,
{
    fn default() -> Self {
        Self::with_registers_and_hasher(
            Registers::with_precision(Precision::default()),
            S::default(),
        )
    }
}

impl<'a, T, S> Extend<&'a T> for HyperLogLog<T, S>
where
    T: 'a + Hash + ?Sized,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<T, S> Extend<T> for HyperLogLog<T, S>
where
    T: Hash,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(&value);
        }
    }
}

impl<T, const N: usize> From<[T; N]> for HyperLogLog<T, RandomState>
where
    T: Hash,
{
    fn from(arr: [T; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<T, S> FromIterator<T> for HyperLogLog<T, S>
where
    T: Hash,
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut hll = HyperLogLog::default();
        hll.extend(iter);
        hll
    }
}
