use natbitset::*;


#[test] fn intersect_pure()
{
    assert_eq!( byteset![]    & byteset![1;8], byteset![] );
    assert_eq!( byteset![1;4] & byteset![5;8], byteset![] );
    assert_eq!( byteset![1;5] & byteset![2,5], byteset![2,5] );
    assert_eq!( byteset![1;3] & byteset![3,4], byteset![3] );
}

#[test] fn intersect_inplace()
{
    let mut bits = byteset![1;4];
    bits &= byteset![1,2,5];
    assert_eq!( bits, byteset![1,2] );
}

#[test] fn union_pure()
{
    assert_eq!( byteset![]    | byteset![1;8], byteset![1;8] );
    assert_eq!( byteset![1;4] | byteset![5;8], byteset![1;8] );
    assert_eq!( byteset![1;5] | byteset![2,5], byteset![1;5] );
    assert_eq!( byteset![1;3] | byteset![3,4], byteset![1;4] );
}

#[test] fn union_inplace()
{
    let mut bits = byteset![1;4];
    bits |= byteset![1,2,5];
    assert_eq!( bits, byteset![1;5] );
}

#[test] fn difference_pure()
{
    assert_eq!( byteset![]    / byteset![1;8], byteset![] );
    assert_eq!( byteset![1;4] / byteset![5;8], byteset![1;4] );
    assert_eq!( byteset![1;5] / byteset![2,5], byteset![1,3,4] );
    assert_eq!( byteset![1;3] / byteset![3,4], byteset![1,2] );
}

#[test] fn difference_inplace()
{
    let mut bits = byteset![1;4];
    bits /= byteset![1,2,5];
    assert_eq!( bits, byteset![3,4] );
}

#[test] fn add_pure()
{
    assert_eq!( byteset![]    + 1, byteset![1] );
    assert_eq!( byteset![1;4] + 5, byteset![1;5] );
    assert_eq!( byteset![1;4] + 4, byteset![1;4] );
    assert_eq!( byteset![1;4] + 99, byteset![1;4] );
}

#[test] fn add_inplace()
{
    let mut bits = byteset![1;4];
    bits += 5;
    assert_eq!( bits, byteset![1;5] );
}

#[test] fn sub_pure()
{
    assert_eq!( byteset![]    - 1, byteset![] );
    assert_eq!( byteset![1;4] - 4, byteset![1;3] );
    assert_eq!( byteset![1;4] - 99, byteset![1;4] );
}

#[test] fn sub_inplace()
{
    let mut bits = byteset![1;5];
    bits -= 5;
    assert_eq!( bits, byteset![1;4] );
}
