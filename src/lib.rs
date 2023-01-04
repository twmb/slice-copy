#![no_std]

//! This crate provides Go style copying / cloning for slices.
//!
//! This crate is for those times where it is easier to adjust slices based off the number of
//! elements copied, as opposed to determining the amount to copy before adjusting slices and
//! finally copying.
//!
//! # Examples
//!
//! We can use `copy` for types that implement `Copy`.
//!
//! ```
//! use slice_copy::copy;
//!
//! let mut l = b"hello".to_vec();
//! let r = b"goodbye".to_vec();
//!
//! let n = copy(&mut l, &r);
//!
//! assert_eq!(n, 5);
//! assert_eq!(l, b"goodb");
//! ```
//!
//! Similarly, we can use `clone` for types that implement `Clone`.
//!
//! ```
//! use slice_copy::clone;
//!
//! let mut l = b"foobarbaz".to_vec();
//! let r = b"biz".to_vec();
//!
//! let n = clone(&mut l, &r);
//!
//! assert_eq!(n, 3);
//! assert_eq!(l, b"bizbarbaz");
//! ```

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
use alloc::vec::Vec;

use core::cmp::min;

/// Copies as many `T` as possible from `src` into `dst`, returning the number of `T` copied. This
/// function is short form for `dst.copy_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
///
/// With the `nightly` feature, `[u8]` is specialized to use [`Read`], which is implemented
/// specially for small slices.
///
/// [`Read`]: https://doc.rust-lang.org/std/primitive.slice.html#impl-Read
///
/// # Examples
///
/// ```
/// use slice_copy::copy;
///
/// let mut l = vec![1 as u8, 2, 3, 4, 5];
/// let r = vec![10, 11, 12];
///
/// let n = copy(&mut l, &r);
///
/// assert_eq!(n, 3);
/// assert_eq!(l, vec![10, 11, 12, 4, 5]);
/// ```
#[inline]
pub fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    let len = min(src.len(), dst.len());
    (&mut dst[..len]).copy_from_slice(&src[..len]);
    len
}

/// Clones as many `T` as possible from `src` into `dst`, returning the number of `T` cloned. This
/// function is short form for `dst.clone_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
///
/// Examples
///
/// ```
/// use slice_copy::clone;
///
/// let mut l = b"left".to_vec();
/// let r = b"right".to_vec();
///
/// let n = clone(&mut l, &r);
///
/// assert_eq!(n, 4);
/// assert_eq!(l, b"righ");
/// ```
#[inline]
pub fn clone<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Clone,
{
    let len = min(src.len(), dst.len());
    (&mut dst[..len]).clone_from_slice(&src[..len]);
    len
}

#[test]
fn test_copy() {
    fn lr() -> (Vec<u8>, Vec<u8>) {
        (b"hello".to_vec(), b"goodbye".to_vec())
    }

    // longer to shorter
    let (mut l, r) = lr();
    assert_eq!(copy(&mut l, &r), 5);
    assert_eq!(l, b"goodb");
    assert_eq!(r, b"goodbye");

    // shorter to longer
    let (l, mut r) = lr();
    assert_eq!(copy(&mut r, &l[..4]), 4);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"hellbye");

    // dst length 0
    let (mut l, r) = lr();
    assert_eq!(copy(&mut l[..0], &r), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");

    // src length 0
    assert_eq!(copy(&mut l, &r[..0]), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");
}
