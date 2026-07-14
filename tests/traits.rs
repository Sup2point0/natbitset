use itertools::*;

use natbitset::*;


#[test] fn default()
{
    let bitset = Bitset::<5>::default();
    assert_eq!( *bitset, 0 );
}

#[test] fn into_iter()
{
    assert_eq!( byteset![].into_iter().collect_vec(), vec![] );
    assert_eq!( byteset![1;8].into_iter().collect_vec(), (1..=8).rev().collect_vec() );
    assert_eq!( byteset![1;7].into_iter().collect_vec(), (1..=7).rev().collect_vec() );
    assert_eq!( byteset![2;8].into_iter().collect_vec(), (2..=8).rev().collect_vec() );

    assert_eq!( byteset![1,3,7].into_iter().collect_vec(), vec![7,3,1] );
    assert_eq!( byteset![2,4,5].into_iter().collect_vec(), vec![5,4,2] );
    
    assert_eq!( Bitset::<31, u32>::all().into_iter().collect_vec(), (1..=31).rev().collect_vec() );
    assert_eq!( Bitset::<32, u32>::all().into_iter().collect_vec(), (1..=32).rev().collect_vec() );
    assert_eq!( Bitset::<63, u64>::all().into_iter().collect_vec(), (1..=63).rev().collect_vec() );
    assert_eq!( Bitset::<64, u64>::all().into_iter().collect_vec(), (1..=64).rev().collect_vec() );
    assert_eq!( Bitset::<127, u128>::all().into_iter().collect_vec(), (1..=127).rev().collect_vec() );
    assert_eq!( Bitset::<128, u128>::all().into_iter().collect_vec(), (1..=128).rev().collect_vec() );
    
    assert_eq!( Bitset::<128, u128>::single(1).into_iter().collect_vec(), vec![1] );
    assert_eq!( Bitset::<128, u128>::single(9).into_iter().collect_vec(), vec![9] );
    assert_eq!( Bitset::<128, u128>::single(128).into_iter().collect_vec(), vec![128] );
}
