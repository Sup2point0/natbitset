use std::*;
use std::collections::HashSet;
use std::error::Error;

use num_traits as nums;

use crate::*;


/// An unordered set representing integers in the range `1..=N`.
/// 
/// You can treat this as a more memory-efficient `HashSet<usize>` specialised for consecutive natural numbers starting from 1. For the rationale behind how this struct works, please visit the [crate root](crate#rationale).
/// 
/// # Type Parameters
/// 
/// - `N` (required): The maximum integer represented by the set.
///   - A `Bitset<N, _>` represents integers `1..=N`, and will ignore integers outside this range.
/// - `Z` (optional): The unsigned integer type used to store the bitflags (e.g. `u8`, `u16`, `usize`).
///   - Defaults to `u8`, which allows the set to represent integers `1..=256`, which should be more than enough to cover most use cases.
/// 
/// ## Notes
/// 
/// - A subtle distinction is that `Z` dictates how many integers the bitset *could* represent, while `N` tells the struct and programmer how many it actually *does* represent.
/// - To optimise space efficiency, you should make `Z` as small as possible for your use case `N`.
///   - However, if you make it too small such that it can’t represent integers up to `N`, you’ll likely encounter overflow errors caused by bitshifting.[^overflow]
/// 
/// [^overflow]: This will hopefully be remedied in future.
/// 
/// # Usage
/// 
/// `Bitset` is designed to be as ergonomic as possible. It does everything a `HashSet<usize>` could, while implementing bitwise operations to make syntax super lightweight.
/// 
/// ## Instantiation
/// 
/// When instantiating a `Bitset`, you’ll need to specify `N`.[^infer]
/// 
/// [^infer]: Unfortunately, `N` cannot be inferred from a collection even if we wanted to, since it’s a `const` generic. However, in future, a macro constructor could achieve this at compile time!
/// 
/// ```rust
/// # use natbitset::*;
/// // A bitset representing numbers 1..=3
/// let bitset = Bitset::<3>::from([1,2,3]);
/// 
/// // A bitset representing numbers 1..=8
/// let bitset = Bitset::<8>::from([1,2,3,4,5,6,7,8]);
/// // or more conveniently:
/// let bitset = Bitset::<8>::all();
/// // or even more conveniently:
/// let bitset = byteset![1;8];
/// 
/// // A bitset representing numbers 1..=1000 (need a larger `Z`!)
/// let bitset = Bitset::<1000, u16>::none();
/// 
/// // Or instantiate manually, passing the bit representation directly:
/// let bitset = Bitset::<4>(0b_0101);
/// let equiv  = Bitset::<4>::from([1,3]);
/// assert_eq!(bitset, equiv);
/// ```
/// 
/// ## Access
/// 
/// To retrieve the integers the bitset represents, use `.members()`:
/// 
/// ```rust
/// # use natbitset::*;
/// use std::collections::HashSet;
/// 
/// let bitset = Bitset::<7>::from([1,3,7]);
/// let digits = bitset.members();
/// 
/// assert_eq!(digits, HashSet::from([1,3,7]));
/// ```
/// 
/// `Bitset<Z>` implements `Deref<Z>`, so the underlying bits can easily be accessed by dereferencing through `*bitset`.
/// 
/// ```rust
/// # use natbitset::*;
/// let mut bitset = Bitset::<8>(0b_0100_1010);
/// assert_eq!(*bitset, 0b_0100_1010);
/// 
/// *bitset += 1;
/// assert_eq!(*bitset, 0b_0100_1011)
/// ```
/// 
/// ## Operations
/// 
/// The union, intersection, difference set operations can be accessed via the `|`, `&`, `/` operations, respectively.
/// 
/// ```rust
/// # use natbitset::*;
/// let left = byteset![1,2,3];
/// let right = byteset![3,4,5];
/// 
/// assert_eq!(left | right, byteset![1,2,3,4,5]);
/// assert_eq!(left & right, byteset![3]);
/// assert_eq!(left / right, byteset![1,2]);
/// ```
/// 
/// Add and remove individual elements via the `+` and `-` operators, respectively.
/// 
/// ```rust
/// # use natbitset::*;
/// let mut bitset = byteset![1,2];
/// 
/// bitset += 3;
/// assert_eq!(bitset, byteset![1,2,3]);
/// 
/// bitset -= 1;
/// assert_eq!(bitset, byteset![2,3]);
/// ```
/// 
/// If you prefer explicit named methods as with `HashSet`, these are all available:
/// 
/// ```rust
/// # use natbitset::*;
/// let mut left = byteset![1,2,3];
/// let mut right = byteset![3,4,5];
/// 
/// assert_eq!(left.union(&right),        byteset![1,2,3,4,5]);
/// assert_eq!(left.intersection(&right), byteset![3]);
/// assert_eq!(left.difference(&right),   byteset![1,2]);
/// 
/// left.insert(4);
/// assert_eq!(left, byteset![1,2,3,4]);
/// 
/// right.remove(&5);
/// assert_eq!(right, byteset![3,4]);
/// ```
/// 
/// These reflect the signatures of `HashSet`, and so in some cases require borrowing.
/// 
/// # Tips
/// 
/// - `Bitset` is **much** more lightweight than `HashSet` – it's only a single integer!
///   - `Bitset` implements `Copy`, so you can pass it around without borrowing.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Bitset<const N: usize, Z = u8>(
    /// The underlying integer used to represent the set. When written in binary, each bit represents whether a number is present in the set (`1` if present, `0` if not).
    /// 
    /// Access this integer by dereferencing a [`Bitset`]:
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// let bitset = Bitset::<4>(0b_1011);
    /// let bits = *bitset;
    /// assert_eq!(bits, 0b_1011);
    /// ```
    pub Z
)
    where Z: PosInt;

// == CONSTRUCTORS == //
/// Constructor methods.
impl<Z: PosInt, const N: usize> Bitset<N,Z>
{
    /// Construct a set with a single integer `int`.
    /// 
    /// # Panics
    /// 
    /// Panics if `n` is not in the range `1..=N` or cannot be converted to a `usize`.
    pub fn single(int: impl AnyInt + fmt::Debug) -> Self
    {
        let Ok(n) = int.try_into() else {
            panic!("Error constructing a singleton `Bitset`: could not convert `{int:?}` to a `usize`")
        };

        if n < 1 || N < n {
            panic!("Error constructing a singleton `Bitset`: received `{int:?}` which is outside of valid range `1..={N}`");
        }

        let z = Z::one() << (n - 1);
        Bitset(z)
    }

    /// Construct a set with no bits enabled.
    /// 
    /// # Usage
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// 
    /// let off = Bitset::<4>::none();
    /// assert_eq!(*off, 0b_0000);
    /// ```
    pub fn none() -> Self {
        Self( Z::zero() )
    }

    /// Construct a set with all bits enabled.
    /// 
    /// # Usage
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// 
    /// let off = Bitset::<4>::all();
    /// assert_eq!(*off, 0b_1111);
    /// ```
    pub fn all() -> Self {
        let z = (1 << N) - 1;
        let z = into_z(z);
        Self(z)
    }
}

impl<Z: PosInt, T: AnyInt, const N: usize, const M: usize> From<[T; M]> for Bitset<N,Z>
{
    fn from(digits: [T; M]) -> Self {
        Self::from_iter(digits)
    }
}

/* NOTE: Z != T because one is the incoming integer type (probably defaulted to `i32`) while the other is the underlying representation type that will be used by the `Bitset` =) */
impl<Z: PosInt, T: AnyInt, const N: usize> FromIterator<T> for Bitset<N,Z>
{
    /// Construct a `Bitset` from an iterator of integers, accepting only those in `1..=N` and ignoring others.
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        let n = nums::cast::<usize, T>(N).unwrap();
        let zero = T::zero();

        Self(
            iter.into_iter()
                .filter(|t| n >= *t && *t > zero)
                .map(|t| Z::one() << into_usize(t - T::one()))
                .sum()
        )
    }
}

/// Construct a [`Bitset::<8, u8>`] with the provided integers.
/// 
/// # Usage
/// 
/// ```rust
/// # use natbitset::*;
/// 
/// // list integers in the set
/// let bitset = byteset![1,2,4];
/// assert_eq!(*bitset, 0b_1011);
/// 
/// // provide bounds of inclusive range
/// let bitset = byteset![3; 7];
/// assert_eq!(*bitset, 0b_0111_1100);
/// ```
#[macro_export]
macro_rules! byteset {
    () => {
        Bitset::<8, u8>(0)
    };

    ( $( $digit:expr ),* $(,)?) => {
        Bitset::<8, u8>::from_iter([ $( $digit ),* ])
    };

    ( $lower:expr ; $upper:expr ) => {
        Bitset::<8, u8>::from_iter($lower ..= $upper)
    };
}

// == TRAITS == //
impl<Z: PosInt, const N: usize> ops::Deref for Bitset<N,Z> {
    type Target = Z;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Z: PosInt, const N: usize> ops::DerefMut for Bitset<N,Z> {
    fn deref_mut(&mut self) -> &mut Z {
        &mut self.0
    }
}

impl<Z: PosInt, const N: usize> Bitset<N,Z> {
    /// Get an iterator over the elements of the set, in descending order.
    pub fn iter(self) -> BitsetIterator<N,Z> {
        self.into_iter()
    }
}

impl<Z: PosInt, const N: usize> IntoIterator for Bitset<N,Z> {
    type Item = usize;
    type IntoIter = BitsetIterator<N,Z>;

    /// Get an iterator over the elements of the set, in descending order.
    fn into_iter(self) -> Self::IntoIter
    {
        BitsetIterator {
            i: N+1,
            residue: *self,
            power_of_2: Z::one() << (N-1),
        }
    }
}
impl<Z: PosInt, const N: usize> IntoIterator for &Bitset<N,Z> {
    type Item = usize;
    type IntoIter = BitsetIterator<N,Z>;

    /// Get an iterator over the elements of the set, in descending order.
    fn into_iter(self) -> Self::IntoIter
    {
        BitsetIterator {
            i: N+1,
            residue: **self,
            power_of_2: Z::one() << (N-1),
        }
    }
}

pub struct BitsetIterator<const N: usize, Z> where Z: PosInt {
    i: usize,
    residue: Z,
    power_of_2: Z,
}
impl<Z: PosInt, const N: usize> Iterator for BitsetIterator<N,Z> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut out = None;

        loop {
            self.i -= 1;
            if self.i == 0 { return None; }

            if self.residue >= self.power_of_2 {
                self.residue -= self.power_of_2;
                out = Some(self.i);
            }

            self.power_of_2 >>= Z::one();

            if out.is_some() { return out; }
        }
    }
}

impl<Z: PosInt, const N: usize> PartialOrd for Bitset<N,Z> {
    /// Checks for a subset relation between `self` and `other`.
    /// 
    /// `self <= other == true` if `self` is a subset of `other`, i.e. all elements of `self` are also elements of `other`.
    /// 
    /// `self < other == true` if `self` is a *strict* subset of `other`, i.e. `self != other`.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let s = *self;
        let r = *other;

        if s == r {
            return Some(cmp::Ordering::Equal)
        }
        else if *(s / r) == Z::zero() {
            return Some(cmp::Ordering::Less)
        }
        else if *(r / s) == Z::zero() {
            return Some(cmp::Ordering::Greater)
        }
        
        None
    }
}

impl<Z: PosInt, const N: usize> fmt::Debug for Bitset<N,Z> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitset {{")?;

        let mut members = self.members_asc().into_iter();

        if let Some(first) = members.next() {
            write!(f, "{first:?}")?;

            for n in members {
                write!(f, ", {n:?}")?;
            }
        }

        write!(f, "}}")
    }
}

// == SET OPERATIONS == //
impl<Z: PosInt, const N: usize> ops::BitOr for Bitset<N,Z> {
    type Output = Self;

    /// Return the union of `self` and `other`, i.e. the combined integers of both sets.
    fn bitor(self, other: Self) -> Self {
        Bitset(*self | *other)
    }
}
impl<Z: PosInt, const N: usize> ops::BitOrAssign for Bitset<N,Z> {
    /// Union `self` with `other`.
    fn bitor_assign(&mut self, other: Self) {
        **self |= *other;
    }
}

impl<Z: PosInt, const N: usize> ops::BitAnd for Bitset<N,Z> {
    type Output = Self;

    /// Return the intersection of `self` and `other`, i.e. the integers that are members of both sets.
    fn bitand(self, other: Self) -> Self {
        Bitset(*self & *other)
    }
}
impl<Z: PosInt, const N: usize> ops::BitAndAssign for Bitset<N,Z> {
    /// Intersect `self` with `other`.
    fn bitand_assign(&mut self, other: Self) {
        **self &= *other;
    }
}

impl<Z: PosInt, const N: usize> ops::Div for Bitset<N,Z> {
    type Output = Self;

    /// Return the difference of `self` and `other`, i.e. the integers that are members of `self` but not `other`.
    fn div(self, other: Self) -> Self::Output {
        Bitset(*self - (*self & *other))
    }
}
impl<Z: PosInt, const N: usize> ops::DivAssign for Bitset<N,Z> {
    /// Remove the elements of `other` from `self`.
    fn div_assign(&mut self, other: Self) {
        let intersect = **self & *other;
        **self -= intersect;
    }
}

impl<Z: PosInt, const N: usize> ops::BitXor for Bitset<N,Z> {
    type Output = Self;

    /// Return the asymmetric difference of `self` and `other`, i.e. the integers that are members of either `self` or `other`, but not both.
    fn bitxor(self, other: Self) -> Self::Output {
        (self | other) / (self & other)
    }
}

impl<Z: PosInt, R: AnyInt, const N: usize> ops::Add<R> for Bitset<N,Z>
{
    type Output = Self;

    /// Add `int` to the set. Does nothing if `int` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when an insertion fails, use [`insert`](Self::insert) or [`try_insert`](Self::try_insert) (but note these are out-of-place).
    fn add(self, int: R) -> Self
    {
        if let Ok(int) = int.try_into()
        && N >= int
        {
            let bit = Z::one() << (int - 1);
            Bitset(*self | bit)
        }
        else {
            self
        }
    }
}
impl<Z: PosInt, R: AnyInt, const N: usize> ops::AddAssign<R> for Bitset<N,Z>
{
    /// Add `int` to the set. Does nothing if `int` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when an insertion fails, use [`insert`](Self::insert) or [`try_insert`](Self::try_insert).
    fn add_assign(&mut self, int: R) {
        *self = *self + int;
    }
}

impl<Z: PosInt, R: AnyInt, const N: usize> ops::Sub<R> for Bitset<N,Z>
{
    type Output = Self;

    /// Remove `int` from the set. Does nothing if `int` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when a removal fails, use [`remove`](Self::remove) or [`try_remove`](Self::try_remove) (but note these are out-of-place).
    /// 
    /// If you wish to be notified when a removal leaves the set empty, use [`remove_nonempty`](Self::remove_nonempty) or [`remove_nonempty_panicking`](Self::remove_nonempty_panicking).
    fn sub(self, int: R) -> Self
    {
        if let Ok(int) = int.try_into()
        && N >= int
        {
            let bit = Z::one() << (int - 1);
            let intersect = *self & bit;
            Bitset(*self - intersect)
        }
        else {
            self
        }
    }
}
impl<Z: PosInt, R: AnyInt, const N: usize> ops::SubAssign<R> for Bitset<N,Z>
{
    /// Remove an integer `other` from the set. Does nothing if `other` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when a removal fails, use [`remove`](Self::remove) or [`try_remove`](Self::try_remove).
    fn sub_assign(&mut self, other: R) {
        *self = *self - other;
    }
}

// == SET METHODS == //
/// Methods following the same signature as `HashSet<>`.
impl<Z: PosInt, const N: usize> Bitset<N,Z>
{
    /// How many integers are in the set?
    pub fn len(self) -> usize
    {
        let mut out = 0;
        let mut residue = *self;
        let mut power_of_2 = Z::one() << (N-1);

        for _ in (0..N).rev() {
            if residue >= power_of_2 {
                residue -= power_of_2;
                out += 1;
            }
            
            power_of_2 >>= Z::one();
        }

        out
    }

    /// Does the set contain `int`?
    /// 
    /// Implemented for compatibility with `HashSet`. You may prefer [`has`](Self::has) which does not require borrowing `int`.
    pub fn contains<R>(self, int: &R) -> bool
        where R: AnyInt
    {
        if let Ok(val) = (*int).try_into() {
            self.iter().any(|n| n == val)
        }
        else {
            false
        }
    }

    /// Add `int` to the set. Returns whether the integer was newly inserted.
    pub fn insert<R>(&mut self, int: R) -> bool
        where R: AnyInt
    {
        let before = *self;
        *self += int;

        *self != before
    }

    /// Try add `int` to the set by casting it into `usize`. Returns an `Ok` indicating whether the integer was newly inserted, or an `Err` if casting failed.
    pub fn try_insert<R>(&mut self, int: R) -> Result<bool, R::Error>
        where R: AnyInt
    {
        let n = int.try_into()?;

        let before = *self;

        if N >= n {
            let bit = Z::one() << (n - 1);
            **self |= bit
        }

        Ok(*self != before)
    }

    /// Remove `int` from the set. Returns whether the integer was a member of the set.
    pub fn remove<R>(&mut self, int: &R) -> bool
        where R: AnyInt
    {
        let before = *self;
        *self -= *int;

        *self != before
    }

    /// Try remove `int` from the set by casting it into `usize`. Returns an `Ok` indicating whether the integer was a member of the set, or an `Err` if casting failed.
    pub fn try_remove<R>(&mut self, int: &R) -> Result<bool, R::Error>
        where R: AnyInt
    {
        let n = (*int).try_into()?;

        let before = *self;
        let bits_before = *before;

        if N >= n {
            let bit = Z::one() << (n - 1);
            let intersect = bits_before & bit;
            **self = bits_before - intersect;
        }

        Ok(*self != before)
    }

    /// Clear the set, removing all integers.
    pub fn clear(&mut self) {
        **self = Z::zero();
    }

    /// Return the union of `self` and `other`, i.e. the combined integers of both sets.
    pub fn union(self, other: &Self) -> Self {
        self | *other
    }

    /// Return the intersection of `self` and `other`, i.e. the integers that are members of both sets.
    pub fn intersection(self, other: &Self) -> Self {
        self & *other
    }

    /// Return the difference of `self` and `other`, i.e. the integers that are members of `self` but not `other`.
    pub fn difference(self, other: &Self) -> Self {
        self / *other
    }

    /// Return the asymmetric difference of `self` and `other`, i.e. the integers that are members of either `self` or `other`, but not both.
    pub fn symmetric_difference(self, other: &Self) -> Self {
        self ^ *other
    }

    /// Do `self` and `other` have no elements in common? (i.e. is the intersection empty?)
    pub fn is_disjoint(self, other: &Self) -> bool {
        *(self & *other) == Z::zero()
    }

    /// Is `self` a subset of `other`?
    /// 
    /// You may wish to use `self <= other` if it's sufficiently unambiguous.
    pub fn is_subset(self, other: &Self) -> bool {
        self <= *other
    }

    /// Is `other` a subset of `self`?
    /// 
    /// You may wish to use `self >= other` if it's sufficiently unambiguous.
    pub fn is_superset(self, other: &Self) -> bool {
        self >= *other
    }

    /// (in-place) Filter `self` to keep only elements that fulfil `predicate`, i.e. remove elements for which `predicate(element) == false`.
    pub fn retain(&mut self, mut predicate: impl FnMut(usize) -> bool)
    {
        let mut res = Z::zero();
        let mut residue = **self;
        let mut power_of_2 = Z::one() << (N-1);

        for i in (1..=N).rev() {
            if residue >= power_of_2 {
                residue -= power_of_2;

                if predicate(i) {
                    res += power_of_2;
                }
            }

            power_of_2 >>= Z::one();
        }

        **self = res;
    }
}

// == QUERY METHODS == //
/// Specialised methods for querying the set.
impl<Z: PosInt, const N: usize> Bitset<N,Z>
{
    /// Is the set empty?
    pub fn is_empty(self) -> bool {
        *self == Z::zero()
    }

    /// Does the set contain only 1 integer?
    pub fn is_single(self) -> bool {
        self.len() == 1
    }

    /// Is the set full? (i.e. it contains every integer in `1..=N`)
    pub fn is_full(self) -> bool {
        self == Self::all()
    }

    /// Does the set contain `int`?
    /// 
    /// Non-borrowed form of [`contains`](Self::contains).
    pub fn has<R>(self, int: R) -> bool
        where R: AnyInt
    {
        if let Ok(val) = int.try_into() {
            self.iter().any(|n| n == val)
        }
        else {
            false
        }
    }

    /// Get the integers in the set.
    /// 
    /// If you only need to iterate over the integers lazily, prefer using [`.iter()`](Self::iter).
    /// 
    /// # Usage
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// use std::collections::HashSet;
    /// 
    /// let bitset = byteset![1,3,7];
    /// let nums = bitset.members();
    /// 
    /// assert_eq!(nums, HashSet::from([1,3,7]));
    /// ```
    pub fn members(self) -> HashSet<usize>
    {
        let mut out = HashSet::new();
        let mut residue = *self;
        let mut power_of_2 = Z::one() << (N-1);

        for i in (1..=N).rev() {
            if residue >= power_of_2 {
                residue -= power_of_2;
                out.insert(i);
            }
            
            power_of_2 >>= Z::one();
        }

        out
    }

    /// Get the integers in the set, sorted in ascending order.
    /// 
    /// # Notes
    /// 
    /// - This is slightly more expensive than [`members_desc`](Self::members_desc) since it requires reversing the output of [`members_desc`](Self::members_desc).
    ///   - This is a limitation of how the integers present in the set are determined from the bitflags.
    pub fn members_asc(self) -> Vec<usize>
    {
        let mut out = self.members_desc();
        out.reverse();
        out
    }

    /// Get the integers in the set, sorted in descending order.
    pub fn members_desc(self) -> Vec<usize>
    {
        self.into_iter().collect::<Vec<usize>>()
    }

    /// Get the minimum integer present in the set, or `None` if the set is empty.
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// assert_eq!(byteset![].minimum(),      None);
    /// assert_eq!(byteset![1,2,6].minimum(), Some(1));
    /// ```
    pub fn minimum(self) -> Option<usize>
    {
        self.iter().min()
    }

    /// Get the maximum integer present in the set, or `None` if the set is empty.
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// assert_eq!(byteset![].maximum(),      None);
    /// assert_eq!(byteset![1,2,6].maximum(), Some(6));
    /// ```
    pub fn maximum(self) -> Option<usize>
    {
        /* NOTE: `.iter()` returns members in decreasing order */
        self.iter().next()
    }

    /// If the set contains only 1 element, return it in a `Some()`, otherwise return `None`.
    /// 
    /// This is more convenient and efficient than `bitset.is_single().then_some(bitset.maximum().unwrap())`, for instance.
    /// 
    /// # Usage
    /// 
    /// ```rust
    /// # use natbitset::*;
    /// assert_eq!( byteset![].only(),    None );
    /// assert_eq!( byteset![1;8].only(), None );
    /// assert_eq!( byteset![1].only(),   Some(1) );
    /// assert_eq!( byteset![8].only(),   Some(8) );
    /// ```
    pub fn only(self) -> Option<usize>
    {
        self.is_single()
            .then_some(self.trailing_zeros() as usize + 1)
    }
}

// == MUTATING METHODS == //
/// Specialised methods for mutating the set.
impl<Z: PosInt, const N: usize> Bitset<N,Z>
    where Z: fmt::Debug
{
    /// Intersect `self` with `other` (in-place). If `self` becomes empty as a result, return an `Err`, leaving `self` unchanged.
    /// 
    /// See [`intersection`](Self::intersection) for more info.
    pub fn intersect_nonempty(&mut self, other: impl Into<Self>) -> Result<(), Box<dyn Error + 'static>>
    {
        let other = other.into();
        let intersect = *self & other;

        if intersect.is_empty() {
            boxerr!(EmptiedBitsetError => "intersecting `{self:?}` and `{other:?}` resulted in empty bitset");
        }

        *self = intersect;

        Ok(())
    }

    /// Intersect `self` with `other` (in-place), panicking if `self` becomes empty as a result.
    /// 
    /// See [`intersection`](Self::intersection) for more info.
    pub fn intersect_nonempty_panicking(&mut self, other: impl Into<Self>)
    {
        if let Err(e) = self.intersect_nonempty(other) { panic!("{e}") }
    }

    /// Remove `int` from `self`, first converting `int` to `usize`. If `self` becomes empty as a result, return an `Err`, leaving `self` unchanged.
    /// 
    /// See [`try_remove`](Self::try_remove) for more info.
    pub fn remove_nonempty<R>(&mut self, int: R) -> Result<(), Box<dyn Error + 'static>>
        where
            R: AnyInt,
            <R as TryInto<usize>>::Error: Error + 'static
    {
        let int = int.try_into()?;
        let diff = *self - int;

        if diff.is_empty() {
            boxerr!(EmptiedBitsetError => "removing `{int:?}` from `{self:?}` resulted in empty bitset");
        }

        *self = diff;

        Ok(())
    }

    /// Remove `int` from `self`, panicking if `self` becomes empty as a result or `int` cannot be converted to `usize`.
    /// 
    /// See [`remove`](Self::remove) for more info.
    pub fn remove_nonempty_panicking<R>(&mut self, int: R)
        where
            R: AnyInt,
            <R as TryInto<usize>>::Error: Error + 'static
    {
        if let Err(e) = self.remove_nonempty(int) { panic!("{e}") }
    }

    /// (in-place) Filter `self` to keep only elements that fulfil `predicate`. If `self` becomes empty as a result, return an `Err`, leaving `self` unchanged.
    /// 
    /// See [`retain`](Self::retain) for more info.
    pub fn retain_nonempty(&mut self,
        predicate: impl FnMut(usize) -> bool,
    ) -> Result<(), Box<dyn Error + 'static>>
    {
        let mut copy = self.clone();
        copy.retain(predicate);

        if copy.is_empty() {
            boxerr!(EmptiedBitsetError => "matching elements of {copy:?} against predicate resulted in empty bitset")
        }

        *self = copy;

        Ok(())
    }

    /// Filter `self` to keep only elements that fulfil `predicate`, panicking if `self` becomes empty as a result.
    /// 
    /// See [`retain`](Self::retain) for more info.
    pub fn retain_nonempty_panicking(&mut self, predicate: impl FnMut(usize) -> bool)
    {
        if let Err(e) = self.retain_nonempty(predicate) { panic!("{e}") }
    }
}


/// Cast a `usize` into a non-negative `Z`.
fn into_z<Z: PosInt>(u: usize) -> Z
{
    nums::cast::<usize, Z>(u).unwrap()
}

/// Cast an integer into a `usize`.
fn into_usize<N: AnyInt>(n: N) -> usize
{
    nums::cast::<N, usize>(n).unwrap()
}
