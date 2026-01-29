use std::*;
use std::error::Error;


#[derive(Clone, Debug)]
pub struct EmptiedBitsetError(pub String);

impl fmt::Display for EmptiedBitsetError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

impl Error for EmptiedBitsetError {}
