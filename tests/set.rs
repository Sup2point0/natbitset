use natbitset::*;


#[test] fn len()
{
    assert_eq!( byteset![].len(), 0 );
    assert_eq!( byteset![1].len(), 1 );
    assert_eq!( byteset![1;7].len(), 7 );
    assert_eq!( byteset![1;8].len(), 8 );
}

#[test] fn contains()
{
    assert!( !byteset![].contains(&0) );
    assert!( !byteset![].contains(&1) );

    assert!( !byteset![1].contains(&0) );
    assert!( byteset![1].contains(&1) );

    assert!( !byteset![1;8].contains(&0) );
    assert!( byteset![1;8].contains(&1) );
    assert!( byteset![1;8].contains(&7) );
    assert!( byteset![1;8].contains(&8) );
    assert!( !byteset![1;8].contains(&9) );
}

#[test] fn insert()
{
    let mut bitset = byteset![];

    assert!( bitset.insert(1) );
    assert_eq!( bitset, byteset![1] );

    assert!( !bitset.insert(1) );
    assert_eq!( bitset, byteset![1] );

    for n in 2..=8 {
        bitset.insert(n);
        assert_eq!( bitset, byteset![1;n] );
    }
    assert_eq!( bitset, byteset![1;8] );
}

#[test] fn try_insert()
{
    let mut bitset = byteset![];

    assert!( bitset.try_insert(-1).is_err() );
    assert_eq!( bitset, byteset![] );

    let res = bitset.try_insert(1);
    assert!( res.is_ok() );
    assert!( res.unwrap() );
    assert_eq!( bitset, byteset![1] );

    for n in 2..=8 {
        let res = bitset.try_insert(n);
        assert!( res.is_ok() );
        assert!( res.unwrap() );
        assert_eq!( bitset, byteset![1;n] );
    }
    assert_eq!( bitset, byteset![1;8] );
}

#[test] fn remove()
{
    let mut bitset = byteset![1;8];

    assert!( bitset.remove(&1) );
    assert_eq!( bitset, byteset![2;8] );

    assert!( !bitset.remove(&1) );
    assert_eq!( bitset, byteset![2;8] );

    for n in 2..=8 {
        assert_eq!( bitset, byteset![n;8] );
        assert!( bitset.remove(&n) );
    }
    assert_eq!( bitset, byteset![] );
}

#[test] fn try_remove()
{
    let mut bitset = byteset![1;8];

    assert!( bitset.try_remove(&-1).is_err() );
    assert_eq!( bitset, byteset![1;8] );

    let res = bitset.try_remove(&1);
    assert!( res.is_ok() );
    assert!( res.unwrap() );
    assert_eq!( bitset, byteset![2;8] );

    for n in 2..=8 {
        assert_eq!( bitset, byteset![n;8] );
        
        let res = bitset.try_remove(&n);
        assert!( res.is_ok() );
        assert!( res.unwrap() );
    }
    assert_eq!( bitset, byteset![] );
}

#[test] fn clear()
{
    for mut bitset in [
        byteset![],
        byteset![1],
        byteset![2],
        byteset![7],
        byteset![8],
        byteset![1;8],
        byteset![1;7],
        byteset![2;8],
    ] {
        bitset.clear();
        assert_eq!( bitset, byteset![] );
    }
}

#[test] fn is_subset()
{
    assert!( byteset![].is_subset(&byteset![]) );
    assert!( byteset![].is_subset(&byteset![1])   ); assert!( !byteset![1].is_subset(&byteset![]) );
    assert!( byteset![].is_subset(&byteset![1;8]) ); assert!( !byteset![1;8].is_subset(&byteset![]) );

    assert!( byteset![1].is_subset(&byteset![1]) );
    assert!( byteset![1].is_subset(&byteset![1;8]) ); assert!( !byteset![1;8].is_subset(&byteset![1]) );

    assert!( byteset![1;8].is_subset(&byteset![1;8]) );
    assert!( byteset![1;7].is_subset(&byteset![1;8]) ); assert!( !byteset![1;8].is_subset(&byteset![1;7]) );
    assert!( byteset![2;8].is_subset(&byteset![1;8]) ); assert!( !byteset![1;8].is_subset(&byteset![2;8]) );
}

#[test] fn is_superset()
{
    assert!( byteset![].is_superset(&byteset![]) );
    assert!( byteset![1].is_superset(&byteset![])   ); assert!( !byteset![].is_superset(&byteset![1]) );
    assert!( byteset![1;8].is_superset(&byteset![]) ); assert!( !byteset![].is_superset(&byteset![1;8]) );

    assert!( byteset![1].is_superset(&byteset![1]) );
    assert!( byteset![1;8].is_superset(&byteset![1]) ); assert!( !byteset![1].is_superset(&byteset![1;8]) );

    assert!( byteset![1;8].is_superset(&byteset![1;8]) );
    assert!( byteset![1;8].is_superset(&byteset![1;7]) ); assert!( !byteset![1;7].is_superset(&byteset![1;8]) );
    assert!( byteset![1;8].is_superset(&byteset![2;8]) ); assert!( !byteset![2;8].is_superset(&byteset![1;8]) );
}

#[test] fn is_disjoint()
{
    assert!( byteset![1].is_disjoint(&byteset![2]) );
    assert!( byteset![2].is_disjoint(&byteset![1]) );

    assert!( byteset![1,2].is_disjoint(&byteset![3,4]) );
    assert!( byteset![3,4].is_disjoint(&byteset![1,2]) );

    assert!( !byteset![1,2].is_disjoint(&byteset![2,3]) );
}

#[test] fn retain()
{
    let mut bitset = byteset![];
    bitset.retain(|_| true);
    assert_eq!( bitset, byteset![] );

    let mut bitset = byteset![1;8];
    bitset.retain(|_| true);
    assert_eq!( bitset, byteset![1;8] );
    
    let mut bitset = byteset![1;8];
    bitset.retain(|_| false);
    assert_eq!( bitset, byteset![] );

    let mut bitset = byteset![1;8];
    bitset.retain(|n| n % 2 == 0);
    assert_eq!( bitset, byteset![2,4,6,8] );
}
