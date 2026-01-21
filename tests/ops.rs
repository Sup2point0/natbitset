use natbitset::*;


#[test] fn intersect_pure()
{
    assert_eq!(
        byteset![] & byteset![1;8],
        byteset![]
    );

    assert_eq!(
        byteset![1;4] & byteset![5;8],
        byteset![]
    );

    assert_eq!(
        byteset![1,2,3,4,5] & byteset![2,5],
        byteset![2,5]
    );

    assert_eq!(
        byteset![1,2,3] & byteset![3,4],
        byteset![3]
    );
}

#[test] fn intersect_inplace()
{
    let mut bits = byteset![1,2,3,4];
    bits &= byteset![1,2,5];
    assert_eq!( bits, byteset![1,2] );
}
