use natbitset::*;


#[test] fn members()
{
    assert_eq!( byteset!(1,2,3).members(), [3,2,1].into_iter().collect() );
    assert_eq!( byteset![1;8].members(), (1..=8).rev().collect() );
}

#[test] fn is_empty()
{
    assert!( Bitset::<8>::none().is_empty() );
    assert!( !Bitset::<8>::all().is_empty() );
}

#[test] fn max()
{
    assert_eq!( Bitset::<8>(0b_0000_0000).max(), None );
    assert_eq!( Bitset::<8>(0b_0000_0011).max(), Some(2) );
    assert_eq!( Bitset::<8>(0b_0000_1011).max(), Some(4) );
    assert_eq!( Bitset::<8>(0b_0100_1011).max(), Some(7) );
    assert_eq!( Bitset::<8>(0b_1100_1011).max(), Some(8) );
}
