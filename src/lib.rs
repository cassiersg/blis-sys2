//! This is a binding for the C API for the [BLIS library](https://github.com/flame/blis).
//!
//! This crate depends on `blis-src` for the actual BLIS dependency.
//! **You have to explicitly depend on [`blis-src`](https://docs.rs/blis-src)** to use this crate.
//! The version of this crate matches the `blis-src` version, not the BLIS version.
//!
//! Related crates and alternatives:
//! - [`blis-src`](https://docs.rs/blis-src): build/link BLIS library
//! - [BLAS/LAPACK crate family](https://github.com/blas-lapack-rs/blas-lapack-rs.github.io/wiki)
//! - [`blis-sys`](https://docs.rs/blis-sys): C binding for BLIS, includes its own BLIS build.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate blis_src as _;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
