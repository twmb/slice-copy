//! This crate provides Go style copying / cloning for slices.
//!
//! For times when you do not want to worry about slices being unequal sizes, this crate is here
//! for you.
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
//! `u8`s are special cased for if either side is short.
//!
//! ```
//! use slice_copy::u8s_copy;
//! 
//! let mut l = vec![1 as u8, 2, 3];
//! let r = vec![10 as u8, 11, 12, 13, 14];
//!
//! let n = u8s_copy(&mut l, &r);
//!
//! assert_eq!(n, 3);
//! assert_eq!(l, vec![10 as u8, 11, 12]);
//! ```
//!
//! Similar to `copy`, we can use `clone`.
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

use std::cmp::min;

/// Copies as many `u8` as possible from `src` into `dst`, returning the number of `u8` copied.
/// This function is a special case of [`copy`]; `u8` slices have a special optimization for
/// reading if they are of length 1.
///
/// [`copy`]: fn.copy.html
#[inline]
pub fn u8s_copy(dst: &mut [u8], src: &[u8]) -> usize {
    // Read for &[u8] special cases 1-length slices.
    use std::io::Read;
    let len = min(src.len(), dst.len());
    (&src[..len]).read(&mut dst[..len]).expect("&[u8] reads never error")
}

/// Copies as many `T` as possible from `src` into `dst`, returning the number of `T` copied. This
/// function is short form for `dst.copy_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
#[inline]
pub fn copy<T>(dst: &mut[T], src: &[T]) -> usize
where T: Copy
{
    let len = min(src.len(), dst.len());
    (&mut dst[..len]).copy_from_slice(&src[..len]);
    len
}

/// Clones as many `T` as possible from `src` into `dst`, returning the number of `T` cloned. This
/// function is short form for `dst.clone_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
#[inline]
pub fn clone<T>(dst: &mut[T], src: &[T]) -> usize
where T: Clone
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
    assert_eq!(u8s_copy(&mut l, &r), 5);
    assert_eq!(l, b"goodb");
    assert_eq!(r, b"goodbye");

    // shorter to longer
    let (l, mut r) = lr();
    assert_eq!(u8s_copy(&mut r, &l[..4]), 4);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"hellbye");

    // dst length 0
    let (mut l, r) = lr();
    assert_eq!(u8s_copy(&mut l[..0], &r), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");

    // src length 0
    assert_eq!(u8s_copy(&mut l, &r[..0]), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");
}

