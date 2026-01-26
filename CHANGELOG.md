# Changelog


## v0.3.0

### Breaking
- Change `HashSet`-isomorphic methods to accept borrowed instead of owned arguments
  - This makes `Bitset` more compatible with `Hashset` for migrations

### New
- `Bitset` derives `Hash`
- `has()` method (non-borrowed form of `contains()`)


## v0.2.0

### Breaking
- `single()` renamed to `only()`
- Change trait bounds on `Add`, `Sub` implementations to accept `Rhs = AnyInt` (`TryInto<usize>`) instead of `Rhs = Into<Z>`

### New
- `contains()`, `insert()`, `remove()`, `clear()` and `intersection()`, `union()`, `difference()` methods to interop with `HashSet`
- `try_insert()`, `try_remove()` methods to protect against `usize` overflow

### Fixes
- Fix `IntoIterator` implementation


## v0.1.0

- Initial release on crates.io!
