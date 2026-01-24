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
}
