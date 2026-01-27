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
}

#[test] fn is_subset()
{
    assert!( byteset![].is_subset(&byteset![]) );
    assert!( byteset![].is_subset(&byteset![1]) );
    assert!( byteset![].is_subset(&byteset![1;8]) );

    assert!( byteset![1].is_subset(&byteset![1]) );
    assert!( byteset![1].is_subset(&byteset![1;8]) );

    assert!( byteset![1;7].is_subset(&byteset![1;8]) );
    assert!( byteset![1;8].is_subset(&byteset![1;8]) );
}

#[test] fn is_superset()
{
    assert!( byteset![].is_superset(&byteset![]) );
    assert!( byteset![1].is_superset(&byteset![]) );
    assert!( byteset![1;8].is_superset(&byteset![]) );
    assert!( byteset![1].is_superset(&byteset![1]) );
    assert!( byteset![1;8].is_superset(&byteset![1]) );
    assert!( byteset![1;8].is_superset(&byteset![1;7]) );
    assert!( byteset![1;8].is_superset(&byteset![1;8]) );
}

#[test] fn is_disjoint()
{
    assert!( byteset![1].is_disjoint(&byteset![2]) );
    assert!( byteset![2].is_disjoint(&byteset![1]) );

    assert!( byteset![1,2].is_disjoint(&byteset![3,4]) );
    assert!( byteset![3,4].is_disjoint(&byteset![1,2]) );

    assert!( !byteset![1,2].is_disjoint(&byteset![2,3]) );
}
