use std::collections::HashSet;

use criterion::*;

use natbitset::*;


criterion_group!(benches, benchmarks);
criterion_main!(benches);


fn benchmarks(c: &mut Criterion)
{
    construct(c);
    insert(c);
    intersection(c);
}


macro_rules! group {
    ($group:expr, $c:ident => $( $id:expr => $func:expr ),* $(,)?) =>
    {
        let mut g = $c.benchmark_group($group);

        $( g.bench_function($id, |b| b.iter(|| $func)); )*

        g.finish();
    }
}

fn construct(c: &mut Criterion)
{
    group!("construction - empty ", c =>
        " HashSet::from([])"     => HashSet::<usize>::from([]),
        " Bitset::<9>::from([])" => Bitset::<9>::from([0; 0]),
        " Bitset::<9>::none()"   => Bitset::<9, u16>::none(),
        " Bitset::<9>::all()"    => Bitset::<9, u16>::all(),
    );

    group!("construction - small ", c =>
        " HashSet::from_iter(1..=9)" => (1..=9).collect::<HashSet<usize>>(),
        " Bitset::from_iter(1..=9)"  => (1..=9).collect::<Bitset::<9, u16>>(),
    );

    group!("construction - large ", c =>
        " HashSet::from_iter(1..=65536)" => (1..=65536).collect::<HashSet<usize>>(),
        " Bitset::from_iter(1..=65536)"  => (1..=65536).collect::<Bitset::<65536, u32>>(),
        " Bitset::<65536>::none()"       => Bitset::<65536, u32>::none(),
        " Bitset::<65536>::all()"        => Bitset::<65536, u32>::all(),
    );

    // VERY LARGE
    // "Bitset::from_iter(1..=2147483647)" = //     (1..=2147483647). ::<Bitset::<2147483647, u3>
    // });
    // "Bitset::<2146483647>::none()" = //     Bitset::<2146483647, u32>::none() /}
    // "Bitset::<2146483647>::all()" = //     Bitset::<2146483647, u32>::all() /}
}

fn insert(c: &mut Criterion)
{
    let mut s = HashSet::new();
    let mut b = byteset![];

    group!("insert - small ", c =>
        " HashSet().insert(1)" => s.insert(1),
        " Bitset().insert(1)"  => b.insert(1),
        " Bitset() += 1"       => b += 1,
    );
}

fn intersection(c: &mut Criterion)
{
    let ls = HashSet::from([1,2,4]);
    let rs = HashSet::from([2,3,5]);

    let lb = byteset![1,2,4];
    let rb = byteset![2,3,5];

    group!("intersect - small ", c =>
        " HashSet[3].intersection()" => ls.intersection(&rs).collect::<HashSet<_>>(),
        " Bitset[3].intersection(&)" => lb.intersection(&rb),
        " Bitset[3].intersection()"  => lb.intersection(rb),
        " Bitset[3] &"        => lb & rb,
    );

    let ls = (1..=42069).collect::<HashSet<_>>();
    let rs = (1729..=69420).collect::<HashSet<_>>();

    let lb = (1..=42069).collect::<Bitset<69420, u32>>();
    let rb = (1729..=69420).collect::<Bitset<69420, u32>>();

    group!("intersect - large ", c =>
        " HashSet[42069].intersection()" => ls.intersection(&rs).collect::<HashSet<_>>(),
        " Bitset[42069].intersection(&)" => lb.intersection(&rb),
        " Bitset[42069].intersection()"  => lb.intersection(rb),
        " Bitset[42069] &"        => lb & rb,
    );
}
