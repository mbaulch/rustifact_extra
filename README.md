# Rustifact extras &emsp; [![Latest Version]][crates.io] [![docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/rustifact_extra.svg
[crates.io]: https://crates.io/crates/rustifact_extra
[docs]: https://docs.rs/rustifact_extra/badge.svg
[docs.rs]: https://docs.rs/rustifact_extra

In future, this crate may provide other extensions to [Rustifact]{https://crates.io/crates/rustifact}, but for now,
it serves to provide jagged array support.

# Motivation
Definition: A *jagged array* is an array with rows of uneven lengths.

Suppose we have a collections of arrays `[T; N1]`, `[T; N2]`..., `[T; Nn]` that we wish
to precalculate at compile time. We can store the elements in a `Vec<Vec<T>>`, or a `[Vec<T>; n]`,
but these are unsuitable for static memory.

The types `JaggedArray` and `BareJaggedArray` allow us to efficiently store jagged arrays in static memory
by pre-populating them in a buildscript with *Rustifact*.

# Types
* `JaggedArray` provides indexing capability at compile time and runtime. Use this type if you're unsure of your requirements.
* `BareJaggedArray` provides indexing capability at compile time. Indexes are injected into runtime as token streams.

# A simple example
build.rs
```rust
use rustifact::ToTokenStream;
use rustifact_extra::JaggedArrayBuilder;

fn main() {
    let mut num_array = JaggedArrayBuilder::new();
    num_array.push(vec![1, 2, 3]);
    num_array.push(vec![4]);
    num_array.push(vec![5, 6]);
    rustifact::write_const!(NUM_ARRAY_LEN, usize, num_array.len());
    rustifact::write_const!(NUM_ARRAY_ELEMS_LEN, usize, num_array.elems_len());
    rustifact::write_static!(NUM_ARRAY, JaggedArray<i32, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN>, &num_array);
}
```

src/main.rs
```rust
rustifact::use_symbols!(NUM_ARRAY, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN);
use rustifact_extra::JaggedArray;

fn main() {
    assert_eq!(NUM_ARRAY[0], [1, 2, 3]);
    assert_eq!(NUM_ARRAY[1], [4]);
    assert_eq!(NUM_ARRAY[2], [5, 6]);
}
```

Cargo.toml
```toml
[package]
## ...

[build-dependencies]
rustifact = "0.9"
rustifact_extra = "0.1"

[dependencies]
rustifact = "0.9"
rustifact_extra = "0.1"
```

# More examples

* [jagged](examples/jagged) Generate a JaggedArray and index it at runtime.

* [barejagged](examples/barejagged) Generate a BareJaggedArray, index it at compile time, and access the indices at runtime. 

# Unsafe code
Unfortunately, the Rust ecosystem (as of late 2023) doesn't provide a mechanism for the creation of slices
in compile-time context. Therefore, *rustifact_extra* does use a small amount `unsafe` code in its *JaggedArray*
and *BareJaggedArray* implementations. Please note that the main *rustifact* crate makes no use of `unsafe`.

# License
rustifact_extra is free software, and is released under the terms of the [Mozilla Public License](https://www.mozilla.org/en-US/MPL/) version 2.0. See [LICENSE](LICENSE).
