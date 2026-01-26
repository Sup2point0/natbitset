# Changelog


## v0.4.0

### Breaking
- `max()` renamed to `maximum()`
  - This is to avoid ambiguity with `self.max(other)` from `PartialOrd`.

### New
- `Bitset` implements `PartialOrd` for checking subset relation
- `minimum()` method (opposite of `maximum()`)
- `is_subset()`, `is_superset()` methods from `HashSet`


## v0.3.0

### Breaking
- Change `HashSet`-isomorphic methods to accept borrowed instead of owned arguments
  - This makes `Bitset` more compatible with `Hashset` for migrations.

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
