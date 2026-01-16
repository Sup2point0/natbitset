use bitset::*;


#[test] fn off()
{
    assert_eq!( *Bitset::<1>::off(), 0 );
    assert_eq!( *Bitset::<2>::off(), 0 );
    assert_eq!( *Bitset::<4>::off(), 0 );
    assert_eq!( *Bitset::<8>::off(), 0 );
}

#[test] fn on()
{
    assert_eq!( *Bitset::<1>::on(), 0b1 );
    assert_eq!( *Bitset::<2>::on(), 0b11 );
    assert_eq!( *Bitset::<4>::on(), 0b1111 );
    assert_eq!( *Bitset::<8>::on(), 0b11111111 );
}
