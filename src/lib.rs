//! A super-lightweight bitset implementation for positive integers.
//! 
//! ## Rationale
//! 
//! You might need this struct if:
//! 
//! - You need to efficiently represent *all* integers in a range `1..=N`
//! - You need to perform set-like operations on those integers
//! - You need better efficiency than `HashSet<usize>`, `HashSet<u8>`, etc.
//! 
//! An example use case will illustrate most clearly.
//! 
//! When solving a Sudoku puzzle, we usually ‘mark’ possible digits for a cell, e.g. “This cell could contain a 1 or 3, but nothing else.” The most intuitive way to represent this would be a `Hashset<usize>` or `Hashset<u8>`, but we can do better.
//! 
//! [`Bitset(z)`](Bitset) is a thin-as-possible abstraction that behaves exactly like a `Hashset<usize>`, but behind-the-scenes only needs to store a *single* number `z`. When written in binary, the bits of `z` represent bitflags for each of the integers in `1..=N` – `1` if the integer is present in the set, `0` if not.
//! 
//! For instance, looking at `Bitset(0b_0000_1011)`, the `1` bits are in positions 1, 2 and 4:
//! 
//! ```text
//! #  | 8765 4321
//! 0b | 0000 1011
//!           ^ ^^
//! ```
//! 
//! This means this `Bitset` represents the set {1, 2, 4}.
//! 
//! If you’ve used enum bitflags in C#, TypeScript, etc. this is intended to work exactly like those, but specifically for a range of integers `1..=N`.
//! 
//! ## Usage
//! 
//! See [`Bitset`] for guidance on how to use the struct.

#[allow(dead_code)]

mod natbitset;
pub use natbitset::Bitset;
