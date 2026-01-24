<h1 align="center"> <code> natbitset </code> </h1>

<div align="center">

  [docs](https://docs.rs/natbitset/latest/natbitset) · [crates.io](https://crates.io/crates/natbitset) · [changelog](CHANGELOG.md)

</div>

A super-lightweight set implementation for consecutive natural numbers `1..=N`.

`Bitset` is a drop-in replacement for `HashSet<usize>`. You might want this if you need:

- To efficiently represent integers in a range `1..=N`
- To perform set-like operations on those integers
- Higher speeds and lower memory usage than `HashSet<usize>`


<br>


## Usage

`natbitset` is available on [crates.io<sup>↗</sup>](https://crates.io/crates/natbitset)! Add to your Rust project:

```bash
> cargo add natbitset
```

Import the `Bitset` struct:

```rust
use natbitset::Bitset;

let bitset = Bitset::<9, u16>::from([1, 3, 7]);
```

For guidance on how to use the struct, and the details behind its implementation, please visit [the documentation on docs.rs<sup>↗</sup>](https://docs.rs/natbitset/latest/natbitset).


<br>


## Features

Flexible instantiation:

```rust
let left   = Bitset::<4>::none();
let right  = Bitset::<4>::all();
```

Implements the same methods as `HashSet`, and then some:

```rust
left.insert(1);
right.remove(4);

let _ = left.intersect_nonempty(right);
let _ = left.max();
let _ = left.is_single();
```

Supports bitwise operations for concise syntax:

```rust
left & right  // intersection
left | right  // union
left / right  // difference
```

Enforce domain with const generic type parameter, and specify backing type for different use cases:

```rust
let bitset = Bitset::<9, u16>::from([1, 2, 4]);
```


<br>


## Performance

`Bitset(z)` represents the set with a *single* integer `z`. The whole data structure is 1 number. This makes it incredibly faster and lighter than a `HashSet<usize>`, especially at scale.

### Memory

| *p* | range         | bitset (*p* bytes) | hashset (*p* bytes / member) |
| :-- | :------------ | :----------------- | :--------------------------- |
| 1   | `1 ..= 8`     | `Bitset<_, u8>`    | `HashSet<u8>`                |
| 2   | `1 ..= 65536` | `Bitset<_, u16>`   | `HashSet<u16>`               |
| 4   | `1 ..= 2^32`  | `Bitset<_, u32>`   | `HashSet<u32>`               |
| 8   | `1 ..= 2^64`  | `Bitset<_, u64>`   | `HashSet<u64>`               |

### Speed

> [!Warning]
> These benchmarks are purely illustrative – results will of course vary hugely depending on environment, background processes, etc. Please use them only as a reference to the differences in magnitude of performance.

| operation     | N     | bitset | hashset | unit | description |
| :------------ | :---- | :----- | :------ | :--- | :---------- |
| construction  | 9     | ~100   | ~100000 | ns   | Constructing a set with members `1..=N` using `FromIterator` |
|               | 65536 | ~10    | ~1000   | μs   |
