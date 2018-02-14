slice-copy
==========

[![Build Status](https://travis-ci.org/twmb/slice-copy.svg?branch=master)](https://travis-ci.org/twmb/slice-copy)  [![Crates.io](https://img.shields.io/crates/v/slice-copy.svg)](https://crates.io/crates/slice-copy) [![Documentation](https://docs.rs/slice-copy/badge.svg)](https://docs.rs/slice-copy/)

Go style copying for slices. For times where you would rather use the amount
copied to adjust your slices as opposed to determining the amount to copy,
adjusting your slices, and finally copying.

```rust
use slice_copy::copy;

let mut l = b"hello".to_vec();
let r = b"goodbye".to_vec();

let n = copy(&mut l, &r);

assert_eq!(n, 5);
assert_eq!(l, b"goodb");
```
