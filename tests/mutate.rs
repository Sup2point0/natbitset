use natbitset::*;


#[test] #[should_panic] fn intersect_nonempty_p1() { byteset![].intersect_nonempty_panicking(byteset![]) }
#[test] #[should_panic] fn intersect_nonempty_p2() { byteset![].intersect_nonempty_panicking(byteset![1;8]) }
#[test] #[should_panic] fn intersect_nonempty_p3() { byteset![1;4].intersect_nonempty_panicking(byteset![5;8]) }

#[test] fn remove_nonempty_1()
{
    for n in 2..=8 {
        byteset![1;n].remove_nonempty_panicking(n);
        byteset![1;n].remove_nonempty_panicking(n);
    }
}

#[test] #[should_panic] fn remove_nonempty_p1() { byteset![].remove_nonempty_panicking(1); }
#[test] #[should_panic] fn remove_nonempty_p2() { byteset![].remove_nonempty_panicking(-1); }
#[test] #[should_panic] fn remove_nonempty_p3() { byteset![].remove_nonempty_panicking(9); }

#[test] #[should_panic] fn retain_nonempty_p1() { byteset![].retain_nonempty_panicking(|_| true) }
#[test] #[should_panic] fn retain_nonempty_p2() { byteset![1].retain_nonempty_panicking(|_| false) }
#[test] #[should_panic] fn retain_nonempty_p3() { byteset![1;8].retain_nonempty_panicking(|_| false) }
