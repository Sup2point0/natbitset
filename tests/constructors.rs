use natbitset::*;


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

#[test] fn from_array()
{
    assert_eq!( *Bitset::<4>::from([1,2,3]), 0b_0111 );
    assert_eq!( *Bitset::<8>::from([1,2,3]), 0b_0111 );
}

#[test] fn from_iter()
{
    assert_eq!( *Bitset::<8>::from_iter(vec![1,2,3]), 0b_0111 );
    assert_eq!( *Bitset::<8>::from_iter(vec![2,4,7,8]), 0b_1100_1010 );
    assert_eq!( *Bitset::<8>::from_iter(vec![2,4,7,99]), 0b_0100_1010 );
}

#[test] fn byteset_macro()
{
    assert_eq!( *byteset![1], 0b_0001 );
    assert_eq!( *byteset![1,2,3], 0b_0111 );
    assert_eq!( *byteset![2,4,7,8], 0b_1100_1010 );

    assert_eq!( byteset![1;8], Bitset::<8, u8>::from([1,2,3,4,5,6,7,8]) );
    assert_eq!( byteset![1;8], Bitset::<8, u8>::from_iter(1..=8) );
}
