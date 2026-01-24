<h1 align="center"> <code> natbitset </code> </h1>

<div align="center">

  [docs](https://docs.rs/natbitset/latest/natbitset) • [crates.io](https://crates.io/crates/natbitset) • [changelog](CHANGELOG.md)

</div>

A super-lightweight set implementation for consecutive natural numbers `1..=N`.


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


## Licence

MIT
