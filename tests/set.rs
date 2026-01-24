use natbitset::*;


#[test] fn contains()
{
    assert!( !byteset![].contains(0) );
    assert!( !byteset![].contains(1) );

    assert!( !byteset![1].contains(0) );
    assert!( byteset![1].contains(1) );

    assert!( !byteset![1;8].contains(0) );
    assert!( byteset![1;8].contains(1) );
    assert!( byteset![1;8].contains(7) );
    assert!( byteset![1;8].contains(8) );
    assert!( !byteset![1;8].contains(9) );
}

#[test] fn insert()
{
    let mut bitset = byteset![];

    assert!( bitset.insert(1) );
    assert_eq!( bitset, byteset![1] );

    assert!( !bitset.insert(1) );
    assert_eq!( bitset, byteset![1] );
}
