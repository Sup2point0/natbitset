use bitset::*;


#[test] fn off()
{
    assert_eq!( *Bitset::<1>::none(), 0 );
    assert_eq!( *Bitset::<2>::none(), 0 );
    assert_eq!( *Bitset::<4>::none(), 0 );
    assert_eq!( *Bitset::<8>::none(), 0 );
}

#[test] fn on()
{
    assert_eq!( *Bitset::<1>::all(), 0b_1 );
    assert_eq!( *Bitset::<2>::all(), 0b_11 );
    assert_eq!( *Bitset::<4>::all(), 0b_1111 );
    assert_eq!( *Bitset::<8>::all(), 0b_1111_1111 );
}

#[test] fn from_iter()
{
    assert_eq!( *Bitset::<8>::from_iter([1,2,3]), 0b_0000_0111 );
    assert_eq!( *Bitset::<8>::from_iter([2,4,7,8]), 0b_1100_1010 );
}
