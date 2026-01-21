use natbitset::*;


#[test] fn is_empty()
{
    assert!( byteset![].is_empty() );
    assert!( !Bitset::<8>::all().is_empty() );
}

#[test] fn is_single()
{
    assert!( byteset![1].is_single() );
    assert!( byteset![8].is_single() );
    assert!( !byteset![1,2].is_single() );
}

#[test] fn is_full()
{}

#[test] fn size()
{
    assert_eq!( byteset![].size(), 0 );
    assert_eq!( byteset![1].size(), 1 );
    assert_eq!( byteset![1;7].size(), 7 );
    assert_eq!( byteset![1;8].size(), 8 );
}

#[test] fn members()
{
    assert_eq!( byteset!(1,2,3).members(), [3,2,1].into_iter().collect() );
    assert_eq!( byteset![1;8].members(), (1..=8).rev().collect() );
}

#[test] fn max()
{
    assert_eq!( Bitset::<8>(0b_0000_0000).max(), None );
    assert_eq!( Bitset::<8>(0b_0000_0011).max(), Some(2) );
    assert_eq!( Bitset::<8>(0b_0000_1011).max(), Some(4) );
    assert_eq!( Bitset::<8>(0b_0100_1011).max(), Some(7) );
    assert_eq!( Bitset::<8>(0b_1100_1011).max(), Some(8) );
}

#[test] fn single()
{}
