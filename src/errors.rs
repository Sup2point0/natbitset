use std::*;
use std::error::Error;


/// An error where performing an operation on a [`Bitset`](crate::Bitset)(s) resulted in an empty bitset.
#[derive(Clone, Debug)]
pub struct EmptiedBitsetError(pub String);

impl fmt::Display for EmptiedBitsetError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for EmptiedBitsetError {}
