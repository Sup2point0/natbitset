use natbitset::*;


#[test] fn intersect_pure()
{
    assert_eq!(
        Bitset::<8>::none() & Bitset::<8>::all(),
        Bitset::<8>::none()
    );

    assert_eq!(
        Bitset::<8>::from_iter([1,2,3,4,5]) & Bitset::<8>::from_iter([2,5]),
        Bitset::<8>::from_iter([2,5])
    );

    assert_eq!(
        Bitset::<8>::from_iter([1,2,3]) & Bitset::<8>::from_iter([3,5]),
        Bitset::<8>::from_iter([3])
    );
}

#[test] fn intersect_inplace()
{
    let mut bs = Bitset::<8>::from_iter([1,2,3,4]);
    bs &= Bitset::<8>::from_iter([1,2,5]);
    assert_eq!( bs, Bitset::<8>::from_iter([1,2]) );
}
