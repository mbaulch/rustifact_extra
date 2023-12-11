// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # Rustifact extras
//!
//! In future, this crate may provide other extensions to *Rustifact*, but for now,
//! it serves to provide jagged array support.
//!
//! # Motivation
//! Definition: A *jagged array* is an array with rows of uneven lengths.
//!
//! Suppose we have a collections of arrays `[T; N1]`, `[T; N2]`..., `[T; Nn]` that we wish
//! to precalculate at compile time. We can store the elements in a `Vec<Vec<T>>`, or a `[Vec<T>; n]`,
//! but these are unsuitable for static memory.
//!
//! The types `JaggedArray` and `BareJaggedArray` allow us to efficiently store jagged arrays in static memory
//! by pre-populating them in a buildscript with *Rustifact*.
//!
//! # Types
//! * `JaggedArray` provides indexing capability at compile time and runtime. Use this type if you're unsure of your requirements.
//! * `BareJaggedArray` provides indexing capability at compile time. Indexes are injected into runtime as token streams.
//!
//! # A simple example
//! build.rs
//! ```no_run
//! use rustifact::ToTokenStream;
//! use rustifact_extra::JaggedArrayBuilder;
//!
//! fn main() {
//!     let mut num_array = JaggedArrayBuilder::new();
//!     num_array.push(vec![1, 2, 3]);
//!     num_array.push(vec![4]);
//!     num_array.push(vec![5, 6]);
//!     rustifact::write_const!(NUM_ARRAY_LEN, usize, num_array.len());
//!     rustifact::write_const!(NUM_ARRAY_ELEMS_LEN, usize, num_array.elems_len());
//!     rustifact::write_static!(NUM_ARRAY, JaggedArray<i32, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN>, &num_array);
//! }
//!```
//!
//!src/main.rs
//! ```no_run
//! rustifact::use_symbols!(NUM_ARRAY, NUM_ARRAY_LEN, NUM_ARRAY_ELEMS_LEN);
//! use rustifact_extra::JaggedArray;
//!
//! fn main() {
//!    assert_eq!(NUM_ARRAY[0], [1, 2, 3]);
//!    assert_eq!(NUM_ARRAY[1], [4]);
//!    assert_eq!(NUM_ARRAY[2], [5, 6]);
//!}
//! ```
//!
//! Cargo.toml
//! ```no_run
//! [package]
//! ## ...
//!
//! [build-dependencies]
//! rustifact = "0.9"
//! rustifact_extra = "0.1"
//!
//! [dependencies]
//! rustifact = "0.9"
//! rustifact_extra = "0.1"
//! ```
//!

use proc_macro2::{Ident, Span};
use rustifact::internal::{quote, TokenStream};
use rustifact::ToTokenStream;
use std::marker::PhantomData;
use std::ops::Index;

// Unfortunately, we must use unsafe code in the implementation of JaggedArray,
// as it requires compile-time generation of slices.
// As of late 2023, no other method exists for creating slices in a const contexts.
pub struct JaggedArray<T, const N: usize, const M: usize> {
    pub elems: [T; M],
    pub offsets: [usize; N],
}

impl<T, const N: usize, const M: usize> JaggedArray<T, N, M> {
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub const fn get_const(&self, index: usize) -> &[T] {
        // *Safety notes *
        // The offsets are precalculated and immutable. The safety of the `end - start` calculation follows
        // from the offsets' monotonicity.
        // The konst crate, for example, provides this functionality but it imposes an unnecessary check
        // on the indices that can significantly increase compile time for large arrays.
        let end = self.offsets[index];
        if index > 0 {
            let start = self.offsets[index - 1];
            unsafe {
                core::slice::from_raw_parts(self.elems.as_ptr().offset(start as _), end - start)
            }
        } else {
            unsafe { core::slice::from_raw_parts(self.elems.as_ptr(), end) }
        }
    }
}

impl<T, const N: usize, const M: usize> Index<usize> for JaggedArray<T, N, M> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 {
            &self.elems[self.offsets[index - 1]..self.offsets[index]]
        } else {
            &self.elems[..self.offsets[index]]
        }
    }
}

// Unfortunately, we must use unsafe code in the implementation of BareJaggedArray,
// as it requires compile-time generation of slices.
// As of late 2023, no other method exists for creating slices in a const contexts.
pub struct BareJaggedArray<T, const M: usize> {
    pub elems: [T; M],
}

#[doc(hidden)]
pub struct VecToArray<T>(Vec<T>)
where
    T: ToTokenStream;

// Copy of an internal function from rustifact's tokens.rs
fn to_toks_slice<T>(sl: &[T], tokens: &mut TokenStream)
where
    T: ToTokenStream,
{
    let mut arr_toks = TokenStream::new();
    for a in sl.iter() {
        let a_toks = a.to_tok_stream();
        let element = quote! { #a_toks, };
        arr_toks.extend(element);
    }
    let element = quote! { [#arr_toks] };
    tokens.extend(element);
}

impl<T> ToTokenStream for VecToArray<T>
where
    T: ToTokenStream,
{
    fn to_toks(&self, tokens: &mut TokenStream) {
        to_toks_slice(&self.0, tokens);
    }
}

pub struct JaggedArrayIndex<T> {
    id: String,
    index: usize,
    phantom: PhantomData<T>,
}

impl<T> JaggedArrayIndex<T> {
    pub fn new(id: &str, index: usize) -> JaggedArrayIndex<T> {
        JaggedArrayIndex {
            id: id.to_owned(),
            index,
            phantom: PhantomData::default(),
        }
    }
}

impl<T> ToTokenStream for JaggedArrayIndex<T> {
    fn to_toks(&self, tokens: &mut TokenStream) {
        let id = Ident::new(&self.id, Span::call_site());
        let index = self.index;
        tokens.extend(quote! { &#id.get_const(#index) });
    }
}

pub struct BareJaggedArrayIndex<T> {
    id: String,
    offset: usize,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T> ToTokenStream for BareJaggedArrayIndex<T> {
    fn to_toks(&self, tokens: &mut TokenStream) {
        let id = Ident::new(&self.id, Span::call_site());
        let offset = self.offset;
        let len = self.len;
        tokens
            .extend(quote! { rustifact_extra::__retrieve_raw_internal(&#id.elems, #offset, #len) });
    }
}

#[doc(hidden)]
pub const fn __retrieve_raw_internal<T>(elems: &[T], offset: usize, len: usize) -> &[T] {
    // * Safety *
    // This function will (or "should") only be called internally with suitable auto-generated parameters.
    unsafe { core::slice::from_raw_parts(elems.as_ptr().offset(offset as _), len) }
}

#[derive(ToTokenStream)]
#[OutType(JaggedArray)]
pub struct JaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    elems: VecToArray<T>,
    offsets: VecToArray<usize>,
}

impl<T> JaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    pub fn new() -> JaggedArrayBuilder<T> {
        JaggedArrayBuilder {
            elems: VecToArray(vec![]),
            offsets: VecToArray(vec![]),
        }
    }

    pub fn push(&mut self, mut elems_push: Vec<T>) {
        self.elems.0.append(&mut elems_push);
        self.offsets.0.push(self.elems.0.len());
    }

    pub fn len(&self) -> usize {
        self.offsets.0.len()
    }

    pub fn elems_len(&self) -> usize {
        self.elems.0.len()
    }
}

impl<T> Index<usize> for JaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 {
            &self.elems.0[self.offsets.0[index - 1]..self.offsets.0[index]]
        } else {
            &self.elems.0[..self.offsets.0[index]]
        }
    }
}

pub struct BareJaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    elems: VecToArray<T>,
    offsets: Vec<usize>,
}

impl<T> ToTokenStream for BareJaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    fn to_toks(&self, tokens: &mut TokenStream) {
        let elems_toks = self.elems.to_tok_stream();
        tokens.extend(quote! { BareJaggedArray { elems: #elems_toks, } });
    }
}

impl<T> BareJaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    pub fn new() -> BareJaggedArrayBuilder<T> {
        BareJaggedArrayBuilder {
            elems: VecToArray(vec![]),
            offsets: vec![],
        }
    }

    pub fn push(&mut self, mut elems_push: Vec<T>) {
        self.elems.0.append(&mut elems_push);
        self.offsets.push(self.elems.0.len());
    }

    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn elems_len(&self) -> usize {
        self.elems.0.len()
    }

    pub fn get_precalc(&self, id: &str, index: usize) -> BareJaggedArrayIndex<T> {
        let offset = if index > 0 {
            self.offsets[index - 1]
        } else {
            0
        };
        let len = self.offsets[index] - offset;
        BareJaggedArrayIndex {
            id: id.to_string(),
            offset,
            len,
            phantom: PhantomData::default(),
        }
    }
}

impl<T> Index<usize> for BareJaggedArrayBuilder<T>
where
    T: ToTokenStream,
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 {
            &self.elems.0[self.offsets[index - 1]..self.offsets[index]]
        } else {
            &self.elems.0[..self.offsets[index]]
        }
    }
}
