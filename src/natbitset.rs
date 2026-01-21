use std::*;
use std::collections::HashSet;

use num_traits as nums;


pub trait PosInt:
    nums::PrimInt
    + nums::Unsigned
    + nums::NumAssign
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + ops::Shl + ops::ShlAssign
    + ops::Shr + ops::ShrAssign
    + iter::Sum
{}

impl<T> PosInt for T where T:
    nums::PrimInt
    + nums::Unsigned
    + nums::NumAssign
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + ops::Shl + ops::ShlAssign
    + ops::Shr + ops::ShrAssign
    + iter::Sum
{}


pub type Byteset<const N: usize> = Bitset::<N, u8>;


/// A set of bitflags representing positive integers in the range `1..=N`.
/// 
/// # Type Parameters
/// 
/// - `N`: The maximum digit stored by the set.
/// - `Z`: The unsigned integer type used to store the bitflags. Defaults to `usize`, but smaller types like `u8` are very likely to be more suitable if `N` is small.
/// 
/// # Usage
/// 
/// ## Instantiation
/// 
/// ## Operations
/// 
/// `Bitset<Z>` implements `Deref<Z>`, so the underlying bits can easily be accessed with `*bitset`.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Bitset<const N: usize, Z = usize>(pub Z)
    where Z: PosInt;

// == CONSTRUCTORS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    /// Construct a `Bitset` with no bits enabled.
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

    /// Construct a `Bitset` with all bits enabled.
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
        Self( (Z::one() << N) - Z::one() )
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
/// let bits = byteset![1,2,4];
/// assert_eq!(*bits, 0b_1011);
/// 
/// // provide bounds (inclusive) inclusive of range
/// let bits = byteset![3; 7];
/// assert_eq!(*bits, 0b_0111_1100);
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

// == BITWISE OPERATIONS == //
impl<Z, const N: usize> ops::BitOr for Bitset<N,Z> where Z: PosInt {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Bitset(*self | *other)
    }
}
impl<Z, const N: usize> ops::BitOrAssign for Bitset<N,Z> where Z: PosInt {
    fn bitor_assign(&mut self, other: Self) {
        **self |= *other
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
        **self &= *other
    }
}

// == QUERY METHODS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
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

    /// How many integers are in the set?
    pub fn size(&self) -> usize
    {
        let mut out = 0;
        let mut val = **self;
        let mut power = Z::one() << N;

        for _ in (1..=N).rev() {
            power >>= Z::one();

            if val >= power {
                val -= power;
                out += 1;
            }
        }

        out
    }

    /// Is the set empty?
    pub fn is_empty(&self) -> bool {
        **self == Z::zero()
    }

    /// Is the set full? (i.e. it contains every integer in `1..=N`)
    pub fn is_full(&self) -> bool {
        *self == Self::all()
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

    /// Intersect `self` with `other`, panicking if the resultant intersection is empty.
    pub fn intersect_nonempty_panicking(&mut self, other: impl Into<Self>)
    {
        match self.intersect_nonempty(other) {
            Err(e) => panic!("{e}"),
            Ok(()) => (),
        }
    }
}


/// Cast a numeric type into `usize`.
fn into_usize<Z>(z: Z) -> usize
    where Z: PosInt
{
    nums::cast::<Z, usize>(z).unwrap()
}
