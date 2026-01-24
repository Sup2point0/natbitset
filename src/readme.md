# Bitflags for Natural Numbers

A super-lightweight bitset implementation for positive integers.[^nat]

[^nat]: I’m of the opinion that $0$ *is* a natural number, but unfortunately “natbitset” flows too smoothly to consider dreadful alternatives like “intbitset” or “posintbitset”...


## Quickstart

```rust
use natbitset::*;

let bitset = Bitset::<8>::from([1,2,3]);

bitset.members();  // => HashSet {1,2,3}

*bitset;  // => 0b_0000_0111

bitset += 4;  // => Bitset(0b_0000_1111)

bitset |= Bitset::<8>::from([4,5,6]);  // => Bitset(0b_0011_1111)

bitset &= Bitset::<8>::from([1,3,7]);  // => Bitset(0b_0000_0101)
```

For more detailed guidance on how to use the struct, as well as an explanation of how it works, please visit the [documentation on docs.rs](https://docs.rs/natbitset/latest)!
