use std::{ops::Deref, *};

use num_traits as nums;


pub trait PosInt:
    nums::PrimInt
    + nums::Unsigned
    + iter::Sum
{}

impl<T> PosInt for T where T:
    nums::PrimInt
    + nums::Unsigned
    + iter::Sum
{}


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitset<const N: usize, Z = usize>(Z)
    where Z: PosInt;

impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    pub fn off() -> Self {
        Self( Z::zero() )
    }

    pub fn on() -> Self {
        Self( (Z::one() << N) - Z::one() )
    }
}

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

impl<Z, const N: usize> Bitset<N,Z> where Z: PosInt
{
    pub fn intersect(&mut self, other: Self) -> () {
        **self = **self & *other;
    }
}

impl<Z, const N: usize> ops::BitAnd for Bitset<N,Z>
    where Z: PosInt
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Bitset(*self & *other)
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


fn into_usize<Z>(z: Z) -> usize
    where Z: PosInt
{
    nums::cast::<Z, usize>(z).unwrap()
}
