use std::*;
use std::collections::HashSet;

use num_traits as nums;


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


pub type Byteset<const N: usize> = Bitset::<N, u8>;


/// A set of bitflags representing positive integers in the range `1..=N`.
/// 
/// # Type Parameters
/// 
/// - `N`: The maximum digit stored by the set.
/// - `Z`: The unsigned integer type used to store the bitflags, e.g. `u8`, `u16`, `usize`. Defaults to `usize`, but smaller types like `u8` are very likely to be more suitable if `N` is small.
/// 
/// # Usage
/// 
/// ## Instantiation
/// 
/// ```rust
/// # use natbitset::*;
/// // A bitset representing numbers 1..=3
/// let bitset = Bitset::<3, u8>::from([1,2,3]);
/// 
/// // A bitset representing numbers 1..=8
/// let bitset = Bitset::<8, u8>::from([1,2,3,4,5,6,7,8]);
/// // or more conveniently:
/// let bitset = Bitset::<8, u8>::all();
/// // or even more conveniently:
/// let bitset = byteset![1;8];
/// 
/// // A bitset representing numbers 1..=1000
/// let bitset = Bitset::<1000, u16>::none();
/// 
/// // Or instantiate manually, passing the bit representation directly:
/// let bitset = Bitset::<4, u8>(0b_0101);
/// assert_eq!(bitset, Bitset::<4, u8>::from([1,3]));
/// ```
/// 
/// ## Access
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
/// assert_eq!(left | right, byteset![1;5]);
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
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Bitset<const N: usize, Z = usize>(
    /// The underlying integer used to represent the set. When written in binary, each bit represents whether a number is present in the set (`1` if present, `0` if not).
    /// 
    /// Access this number by dereferencing a [`Bitset`]:
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

impl<Z, const N: usize, const M: usize> From<[Z; M]> for Bitset<N,Z> where Z: PosInt
{
    fn from(digits: [Z; M]) -> Self {
        Self::from_iter(digits)
    }
}

impl<Z, const N: usize> FromIterator<Z> for Bitset<N,Z> where Z: PosInt
{
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = Z>
    {
        Self(
            iter.into_iter()
                .map(|z| Z::one() << into_usize(z) - 1)
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
    ( $( $digit:expr ),* $(,)?) =>
    {
        Bitset::<8, u8>::from_iter([ $( $digit ),* ])
    };

    ( $lower:expr ; $upper:expr ) =>
    {
        Bitset::<8, u8>::from_iter($lower ..= $upper)
    }
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

impl<Z, const N: usize> IntoIterator for Bitset<N,Z> where Z: PosInt {
    type Item = usize;
    type IntoIter = BitsetIterator<N,Z>;

    fn into_iter(self) -> Self::IntoIter
    {
        BitsetIterator {
            i: N+1,
            z: *self,
            power_of_2: Z::one() << N,
        }
    }
}

pub struct BitsetIterator<const N: usize, Z> where Z: PosInt {
    i: usize,
    z: Z,
    power_of_2: Z,
}
impl<Z, const N: usize> Iterator for BitsetIterator<N,Z> where Z: PosInt {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop {
            self.i -= 1;
            if self.i == 0 { return None; }

            self.power_of_2 >>= Z::one();

            if self.z >= self.power_of_2 {
                self.z -= self.power_of_2;
                return Some(self.i);
            }
        }
    }
}

// == SET OPERATIONS == //
impl<Z, R, const N: usize> ops::Add<R> for Bitset<N,Z>
    where Z: PosInt, R: Into<Z>,
{
    type Output = Self;

    /// Add an integer `other` to the set. Does nothing if the integer is not in the range `1..=N`.
    fn add(self, other: R) -> Self
    {
        let other = other.into();

        if N >= into_usize(other) {
            let bit = Z::one() << (other - Z::one());
            Bitset(*self | bit)
        }
        else {
            self
        }
    }
}
impl<Z, R, const N: usize> ops::AddAssign<R> for Bitset<N,Z>
    where Z: PosInt, R: Into<Z>,
{
    fn add_assign(&mut self, other: R) {
        *self = *self + other;
    }
}

impl<Z, R, const N: usize> ops::Sub<R> for Bitset<N,Z>
    where Z: PosInt, R: Into<Z>,
{
    type Output = Self;

    fn sub(self, other: R) -> Self
    {
        let other = other.into();

        if N >= into_usize(other) {
            let bit = Z::one() << (other - Z::one());
            let intersect = *self & bit;
            Bitset(*self - intersect)
        }
        else {
            self
        }
    }
}
impl<Z, R, const N: usize> ops::SubAssign<R> for Bitset<N,Z>
    where Z: PosInt, R: Into<Z>,
{
    fn sub_assign(&mut self, other: R) {
        *self = *self - other;
    }
}

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

// == QUERY METHODS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    /// Is the set empty?
    pub fn is_empty(&self) -> bool {
        **self == Z::zero()
    }

    /// Does the set contain only 1 element?
    pub fn is_single(&self) -> bool {
        self.size() == 1
    }

    /// Is the set full? (i.e. it contains every integer in `1..=N`)
    pub fn is_full(&self) -> bool {
        *self == Self::all()
    }

    /// How many integers are in the set?
    pub fn size(&self) -> usize
    {
        let mut out = 0;
        let mut val = **self;
        let mut power = Z::one() << (N-1);

        for _ in (0..N).rev() {
            if val >= power {
                val -= power;
                out += 1;
            }
            
            power >>= Z::one();
        }

        out
    }

    /// Get the integers present in the set.
    pub fn members(&self) -> HashSet<usize>
    {
        let mut out = HashSet::new();
        let mut val = **self;
        let mut power = Z::one() << (N-1);

        for i in (1..=N).rev() {
            if val >= power {
                val -= power;
                out.insert(i);
            }
            
            power >>= Z::one();
        }

        out
    }

    /// Get the maximum integer present in the set, or `0` if the set is empty.
    pub fn max(&self) -> Option<usize>
    {
        (0..N)
            .rev()
            .filter_map(|n| {
                let pow = Z::one() << n;
                let present = (**self & pow) > Z::zero();
                present.then_some(n+1)
            })
            .next()
    }

    pub fn single(&self) -> Option<usize>
    {
        self.is_single().then_some(self.trailing_zeros() as usize + 1)
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

/// Cast a `Z` into a `usize`.
fn into_usize<Z>(z: Z) -> usize
    where Z: PosInt
{
    nums::cast::<Z, usize>(z).unwrap()
}
