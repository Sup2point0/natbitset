use natbitset::*;


#[test] #[should_panic] fn intersect_nonempty_1() { byteset![].intersect_nonempty_panicking(byteset![]) }
#[test] #[should_panic] fn intersect_nonempty_2() { byteset![].intersect_nonempty_panicking(byteset![1;8]) }
#[test] #[should_panic] fn intersect_nonempty_3() { byteset![1;4].intersect_nonempty_panicking(byteset![5;8]) }

#[test] #[should_panic] fn retain_nonempty_1() { byteset![].retain_nonempty_panicking(|_| true) }
#[test] #[should_panic] fn retain_nonempty_2() { byteset![1].retain_nonempty_panicking(|_| false) }
#[test] #[should_panic] fn retain_nonempty_3() { byteset![1;8].retain_nonempty_panicking(|_| false) }
