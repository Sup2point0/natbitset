use natbitset::*;


#[test] fn is_empty()
{
    assert!( byteset![].is_empty() );

    assert!( !byteset![1].is_empty() );
    assert!( !byteset![1;8].is_empty() );
}

#[test] fn is_single()
{
    assert!( byteset![1].is_single() );
    assert!( byteset![8].is_single() );

    assert!( !byteset![1,2].is_single() );
    assert!( !byteset![1;8].is_single() );
}

#[test] fn is_full()
{
    assert!( byteset![1;8].is_full() );

    assert!( !byteset![].is_full() );
    assert!( !byteset![1,2].is_full() );
}

#[test] fn has()
{
    assert!( !byteset![].has(0) );
    assert!( !byteset![].has(1) );

    assert!( !byteset![1].has(0) );
    assert!( byteset![1].has(1) );

    assert!( !byteset![1;8].has(0) );
    assert!( byteset![1;8].has(1) );
    assert!( byteset![1;8].has(7) );
    assert!( byteset![1;8].has(8) );
    assert!( !byteset![1;8].has(9) );
}

#[test] fn members()
{
    assert_eq!( byteset![].members(), std::collections::HashSet::new() );
    assert_eq!( byteset![1,2].members(), [2,1].into_iter().collect() );
    assert_eq!( byteset![1;8].members(), (1..=8).rev().collect() );
}

#[test] fn max()
{
    assert_eq!( byteset![].max(), None );
    assert_eq!( byteset![1,2].max(), Some(2) );
    assert_eq!( byteset![1,2,4].max(), Some(4) );
    assert_eq!( byteset![1,2,4,7].max(), Some(7) );
    assert_eq!( byteset![1,2,4,7,8].max(), Some(8) );
}

#[test] fn single()
{
    assert_eq!( byteset![1].only(), Some(1) );
    assert_eq!( byteset![8].only(), Some(8) );

    assert_eq!( byteset![].only(), None );
    assert_eq!( byteset![1,2].only(), None );
    assert_eq!( byteset![1;8].only(), None );
}
