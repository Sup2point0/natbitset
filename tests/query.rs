use bitset::*;


#[test] fn is_empty()
{
    assert!( Bitset::<8>::none().is_empty() );
    assert!( !Bitset::<8>::all().is_empty() );
}

#[test] fn max()
{
    assert_eq!( Bitset::<8>::none().max(), 0 );
    assert_eq!( Bitset::<8>(0b_0000_0011).max(), 2 );
    assert_eq!( Bitset::<8>(0b_0000_1011).max(), 8 );
    assert_eq!( Bitset::<8>(0b_0100_1011).max(), 64 );
}
