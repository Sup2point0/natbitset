use std::collections::HashSet;

use criterion::*;

use natbitset::*;


criterion_group!(benches, benchmarks);
criterion_main!(benches);


fn benchmarks(c: &mut Criterion)
{
    construct(c);
    insert(c);
}


macro_rules! group {
    ($group:expr $( $id:expr => $func:expr ),* $(,)?) =>
    {
        let mut g = c.benchmark_group($group);
        $( g.bench_function($id, |b| b.iter(|| $func)) );*
        g.finish()
    }
}

fn construct(c: &mut Criterion)
{
    group!("construction - empty "
        "HashSet::from([])"     => HashSet::<usize>::from([]),
        "Bitset::<9>::from([])" => Bitset::<9>::from([0; 0]),
        "Bitset::<9>::none()"   => Bitset::<9, u16>::none(),
        "Bitset::<9>::all()"    => Bitset::<9, u16>::all(),
    );

    group!("construction - small "
        "HashSet::from_iter(1..=9)" => (1..=9).collect::<HashSet<usize>>(),
        "Bitset::from_iter(1..=9)"  => (1..=9).collect::<Bitset::<9, u16>>(),
    );

    group!("construction - large "
        "HashSet::from_iter(1..=65536)" => (1..=65536).collect::<HashSet<usize>>(),
        "Bitset::from_iter(1..=65536)"  => (1..=65536).collect::<Bitset::<65536, u32>>(),
        "Bitset::<65536>::none()"       => Bitset::<65536, u32>::none(),
        "Bitset::<65536>::all()"        => Bitset::<65536, u32>::all(),
    );

    // VERY LARGE
    // "Bitset::from_iter(1..=2147483647)" = //     (1..=2147483647). ::<Bitset::<2147483647, u3>
    // });
    // "Bitset::<2146483647>::none()" = //     Bitset::<2146483647, u32>::none() /}
    // "Bitset::<2146483647>::all()" = //     Bitset::<2146483647, u32>::all() /}

    g.finish();
}

fn insert(c: &mut Criterion)
{
    let s = HashSet::new();
    let b = Bitset::<9>::none();

    group!("insert - small "
        "HashSet().insert(1)" => s.insert(1),
        "Bitset().insert(1)"  => b.insert(1),
        "Bitset().insert(1)"  => b += 1,
    );
}
