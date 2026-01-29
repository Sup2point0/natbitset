# Changelog


## v0.5.0

### Breaking
- Improve error interfaces
  - `EmptiedBitsetError` struct
  - Change return signatures from `Result<(), Box<dyn Error + 'static>>`

### New
- New methods:
  - `remove_nonempty()`
  - `remove_nonempty_panicking()`
  - `retain_nonempty()`
  - `retain_nonempty_panicking()`

### Fixes
- Fix errors and ambiguities in docs


## v0.4.1

### New
- New methods:
  - `single()` for constructing singleton set
  - `retain()` from `HashSet` for filtering out elements

### Fixes
- `iter()` is public instead of private


## v0.4.0

### Breaking
- `max()` renamed to `maximum()`
  - This is to avoid ambiguity with `self.max(other)` from `PartialOrd`.

### New
- `Bitset` implements `PartialOrd` for checking subset relation
- `Bitset` supports `bitset ^ bitset` for symmetric difference
- New methods:
  - `minimum()` method (opposite of `maximum()`)
  - `symmetric_difference()`
  - `is_subset()`
  - `is_superset()`
  - `is_disjoint()`


## v0.3.0

### Breaking
- Change `HashSet`-isomorphic methods to accept borrowed instead of owned arguments
  - This makes `Bitset` more compatible with `Hashset` for migrations.

### New
- `Bitset` derives `Hash`
- New methods:
  - `has()` (non-borrowed form of `contains()`)


## v0.2.0

### Breaking
- `single()` renamed to `only()`
- Change trait bounds on `Add`, `Sub` implementations to accept `Rhs = AnyInt` (`TryInto<usize>`) instead of `Rhs = Into<Z>`

### New
- New methods:
  - To interop with `HashSet`:
    - `contains()`
    - `insert()`
    - `remove()`
    - `clear()`
    - `intersection()`
    - `union()`
    - `difference()`
  - To protect against `usize` overflow:
    - `try_insert()`
    - `try_remove()`

### Fixes
- Fix `IntoIterator` implementation


## v0.1.0

- Initial release on crates.io!
