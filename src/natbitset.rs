use std::*;
use std::collections::HashSet;

use num_traits as nums;


/// Any integer type, such as `i32`, `usize`, `isize`.
pub trait AnyInt:
    TryInto<usize>
    + nums::PrimInt
    + nums::NumAssign
    + iter::Sum
{}

impl<T> AnyInt for T where T:
    TryInto<usize>
    + nums::PrimInt
    + nums::NumAssign
    + iter::Sum
{}

/// A positive integer type, such as `u8`, `u16`, `usize`.
pub trait PosInt:
    nums::PrimInt
    + nums::Unsigned
    + nums::NumAssign
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + ops::Shl<Output = Self> + ops::ShlAssign
    + ops::Shr<Output = Self> + ops::ShrAssign
    + iter::Sum
{}

impl<T> PosInt for T where T:
    nums::PrimInt
    + nums::Unsigned
    + nums::NumAssign
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + ops::Shl<Output = Self> + ops::ShlAssign
    + ops::Shr<Output = Self> + ops::ShrAssign
    + iter::Sum
{}


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
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
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

impl<Z, T, const N: usize, const M: usize> From<[T; M]> for Bitset<N,Z>
    where Z: PosInt, T: AnyInt
{
    fn from(digits: [T; M]) -> Self {
        Self::from_iter(digits)
    }
}

/* NOTE: Z != T because one is the incoming integer type (probably defaulted to `i32`) while the other is the underlying representation type that will be used by the `Bitset` =) */
impl<Z, T, const N: usize> FromIterator<T> for Bitset<N,Z>
    where Z: PosInt, T: AnyInt
{
    /// Construct a `Bitset` from an iterator of integers, accepting only those in `1..=N` and ignoring others.
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        let n = nums::cast::<usize, T>(N).unwrap();
        let zero = T::zero();

        Self(
            iter.into_iter()
                .filter_map(|t|
                    (n >= t && t > zero).then(||
                        Z::one() << into_usize(t - T::one())
                    )
                )
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
/// // provide bounds (inclusive) inclusive of range
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
impl<Z, const N: usize> ops::Deref for Bitset<N,Z> where Z: PosInt {
    type Target = Z;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Z, const N: usize> ops::DerefMut for Bitset<N,Z> where Z: PosInt {
    fn deref_mut(&mut self) -> &mut Z {
        &mut self.0
    }
}

impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt {
    /// Get an iterator over the members of the set in 
    fn iter(self) -> BitsetIterator<N,Z> {
        self.into_iter()
    }
}

impl<Z, const N: usize> IntoIterator for Bitset<N,Z> where Z: PosInt {
    type Item = usize;
    type IntoIter = BitsetIterator<N,Z>;

    fn into_iter(self) -> Self::IntoIter
    {
        BitsetIterator {
            i: N+1,
            residue: *self,
            power_of_2: Z::one() << (N-1),
        }
    }
}
impl<'l, Z, const N: usize> IntoIterator for &'l Bitset<N,Z> where Z: PosInt {
    type Item = usize;
    type IntoIter = BitsetIterator<N,Z>;

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
impl<Z, const N: usize> Iterator for BitsetIterator<N,Z> where Z: PosInt {
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

impl<Z, const N: usize> fmt::Debug for Bitset<N,Z> where Z: PosInt {
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
impl<Z, const N: usize> ops::BitOr for Bitset<N,Z> where Z: PosInt {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Bitset(*self | *other)
    }
}
impl<Z, const N: usize> ops::BitOrAssign for Bitset<N,Z> where Z: PosInt {
    fn bitor_assign(&mut self, other: Self) {
        **self |= *other;
    }
}

impl<Z, const N: usize> ops::BitAnd for Bitset<N,Z> where Z: PosInt {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Bitset(*self & *other)
    }
}
impl<Z, const N: usize> ops::BitAndAssign for Bitset<N,Z> where Z: PosInt {
    fn bitand_assign(&mut self, other: Self) {
        **self &= *other;
    }
}

impl<Z, const N: usize> ops::Div for Bitset<N,Z> where Z: PosInt {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Bitset(*self - (*self & *other))
    }
}
impl<Z, const N: usize> ops::DivAssign for Bitset<N,Z> where Z: PosInt {
    fn div_assign(&mut self, other: Self) {
        let intersect = **self & *other;
        **self -= intersect;
    }
}

impl<Z, R, const N: usize> ops::Add<R> for Bitset<N,Z>
    where Z: PosInt, R: AnyInt,
{
    type Output = Self;

    /// Add an integer `other` to the set. Does nothing if `other` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when an insertion fails, use [`insert`](Self::insert) or [`try_insert`](Self::try_insert) (but note these are out-of-place).
    fn add(self, other: R) -> Self
    {
        if let Ok(other) = other.try_into()
        && N >= other
        {
            let bit = Z::one() << (other - 1);
            Bitset(*self | bit)
        }
        else {
            self
        }
    }
}
impl<Z, R, const N: usize> ops::AddAssign<R> for Bitset<N,Z>
    where Z: PosInt, R: AnyInt,
{
    /// Add an integer `other` to the set. Does nothing if `other` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when an insertion fails, use [`insert`](Self::insert) or [`try_insert`](Self::try_insert).
    fn add_assign(&mut self, other: R) {
        *self = *self + other;
    }
}

impl<Z, R, const N: usize> ops::Sub<R> for Bitset<N,Z>
    where Z: PosInt, R: AnyInt,
{
    type Output = Self;

    /// Remove an integer `other` from the set. Does nothing if `other` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when a removal fails, use [`remove`](Self::remove) or [`try_remove`](Self::try_remove) (but note these are out-of-place).
    fn sub(self, other: R) -> Self
    {
        if let Ok(other) = other.try_into()
        && N >= other
        {
            let bit = Z::one() << (other - 1);
            let intersect = *self & bit;
            Bitset(*self - intersect)
        }
        else {
            self
        }
    }
}
impl<Z, R, const N: usize> ops::SubAssign<R> for Bitset<N,Z>
    where Z: PosInt, R: AnyInt,
{
    /// Remove an integer `other` from the set. Does nothing if `other` is not in the range `1..=N`.
    /// 
    /// If you wish to be notified when a removal fails, use [`remove`](Self::remove) or [`try_remove`](Self::try_remove).
    fn sub_assign(&mut self, other: R) {
        *self = *self - other;
    }
}

// == SET METHODS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
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

    /// Remove `int` from the set. Returns whether the integer was present in the set.
    pub fn remove<R>(&mut self, int: &R) -> bool
        where R: AnyInt
    {
        let before = *self;
        *self -= *int;

        *self != before
    }

    /// Try remove `int` from the set by casting it into `usize`. Returns an `Ok` indicating whether the integer was present in the set, or an `Err` if casting failed.
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

    /// Return the intersection of the set with `other`.
    pub fn intersection(self, other: &Self) -> Self {
        self & *other
    }

    /// Return the union of the set with `other`.
    pub fn union(self, other: &Self) -> Self {
        self | *other
    }

    /// Return the difference of the set with `other`, i.e. the integers that are present in `self` but not in `other`.
    pub fn difference(self, other: &Self) -> Self {
        self / *other
    }
}

// == QUERY METHODS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
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

    /// Get the integers present in the set.
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

    /// Get the integers present in the set, sorted in ascending order.
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

    /// Get the integers present in the set, sorted in descending order.
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
    /// This is more convenient and efficient than `bitset.is_single().then_some(bitset.max().unwrap())`.
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
impl<Z, const N: usize> Bitset<N,Z>
    where Z: PosInt + fmt::Debug
{
    /// Intersect `self` with `other`, returning an `Err` if the intersection is empty.
    pub fn intersect_nonempty(&mut self, other: impl Into<Self>) -> Result<(), String>
    {
        let other = other.into();
        let intersect = *self & other;

        if intersect.is_empty() {
            return Err(format!(
                "Intersecting bitsets `{self:?}` and `{other:?}` resulted in empty bitset!"
            ));
        }

        *self = intersect;

        Ok(())
    }

    /// Intersect `self` with `other`, panicking with debug output if the resultant intersection is empty.
    pub fn intersect_nonempty_panicking(&mut self, other: impl Into<Self>)
    {
        match self.intersect_nonempty(other) {
            Err(e) => panic!("{e}"),
            Ok(()) => (),
        }
    }
}


/// Cast a `usize` into a `Z`.
fn into_z<Z>(u: usize) -> Z
    where Z: PosInt
{
    nums::cast::<usize, Z>(u).unwrap()
}

/// Cast an integer into a `usize`.
fn into_usize<Z>(n: Z) -> usize
    where Z: AnyInt
{
    nums::cast::<Z, usize>(n).unwrap()
}
