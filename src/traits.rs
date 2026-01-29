use std::*;
use std::error::Error;

use num_traits as nums;


/// Any integer type, such as `i32`, `usize`, `isize`.
pub trait AnyInt:
    TryInto<usize>
    + nums::PrimInt
    + nums::NumAssign
    + iter::Sum
// where
//     <Self as TryInto<usize>>::Error: Error
{}

impl<T> AnyInt for T where
    T:
        TryInto<usize>
        + nums::PrimInt
        + nums::NumAssign
        + iter::Sum,
    <T as TryInto<usize>>::Error:
        Error
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
