# natbitset

A super-lightweight bitset implementation for positive integers.


## Quickstart

```rust
use natbitset::*;

// Construct a bitset:
let bitset = Bitset::<8>::from([1,2,3]);

// Retrieve the integers present in the set:
bitset.members();
// => HashSet {1,2,3}

// Get the underlying bit representation:
*bitset;
// => 0b_0000_0111

// Add an element:
bitset += 4;
// => Bitset(0b_0000_1111)

// Union with another set:
bitset |= Bitset::<8>::from([4,5,6]);
// => Bitset(0b_0011_1111)

// Intersect with another set:
bitset &= Bitset::<8>::from([1,3,7]);
// => Bitset(0b_0000_0101)
```

For more detailed guidance on how to use the struct, as well as an explanation of how it works, please visit the [documentation on docs.rs](https://docs.rs/natbitset/latest)!
