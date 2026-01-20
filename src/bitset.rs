use std::{ops::Deref, *};

use num_traits as nums;


pub trait PosInt:
    nums::PrimInt
    + nums::Unsigned
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + iter::Sum
    + fmt::Debug
{}

impl<T> PosInt for T where T:
    nums::PrimInt
    + nums::Unsigned
    + ops::BitOr + ops::BitOrAssign
    + ops::BitAnd + ops::BitAndAssign
    + iter::Sum
    + fmt::Debug
{}


#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Bitset<const N: usize, Z = usize>(pub Z)
    where Z: PosInt;

// == CONSTRUCTORS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    pub fn none() -> Self {
        Self( Z::zero() )
    }

    pub fn all() -> Self {
        Self( (Z::one() << N) - Z::one() )
    }
}

impl<Z, const N: usize> FromIterator<Z> for Bitset<N,Z>
    where Z: PosInt
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

#[macro_export]
macro_rules! byteset {
    ( $( $digit:expr ),* $(,)?) => {
        Bitset::<8>::from_iter([ $( $digit ),* ])
    };
}

// == TRAITS == //
impl<Z, const N: usize> Deref for Bitset<N,Z>
    where Z: PosInt
{
    type Target = Z;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Z, const N: usize> ops::DerefMut for Bitset<N,Z>
    where Z: PosInt
{
    fn deref_mut(&mut self) -> &mut Z {
        &mut self.0
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
    pub fn is_empty(&self) -> bool {
        **self == Z::zero()
    }

    pub fn max(&self) -> Z {
        (1..N).rev()
            .map(|n| into_z::<usize, Z>((2 as usize).pow(n as u32)))
            .filter(|n| (*n & **self) > Z::zero())
            .next()
            .unwrap_or(Z::zero())
    }
}

// == MUTATING METHODS == //
impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    pub fn intersect_nonempty(&mut self, other: impl Into<Self>)
    {
        let other = other.into();
        let intersect = *self & other;

        if intersect.is_empty() {
            panic!("Intersecting bitsets `{self:?}` and `{other:?}` resulted in empty bitset!")
        }

        *self = intersect;
    }
}


fn into_z<T,Z>(t: T) -> Z
    where T: PosInt, Z: PosInt
{
    nums::cast::<T,Z>(t).unwrap()
}

fn into_usize<Z>(z: Z) -> usize
    where Z: PosInt
{
    nums::cast::<Z, usize>(z).unwrap()
}
