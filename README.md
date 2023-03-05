# blis-sys2 [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url]

This is a binding for the C API for the [BLIS library](https://github.com/flame/blis).

This crate depends on `blis-src` for the actual BLIS dependency.
**You have to explicitly depend on [`blis-src`](https://docs.rs/blis-src)** to use this crate.
The version of this crate matches the `blis-src` version, not the BLIS version.

Related crates and alternatives:
- [`blis-src`](https://docs.rs/blis-src): build/link BLIS library
- [BLAS/LAPACK crate family](https://github.com/blas-lapack-rs/blas-lapack-rs.github.io/wiki)
- [`blis-sys`](https://docs.rs/blis-sys): C binding for BLIS, includes its own BLIS build.

# License

## Apache 2.0/MIT

All original work licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
     at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[blis]: https://github.com/flame/blis
[blis-src]: https://crates.io/crates/blis-src

[package-img]: https://img.shields.io/crates/v/blis-sys2.svg
[package-url]: https://crates.io/crates/blis-sys2
[documentation-img]: https://docs.rs/blis-sys2/badge.svg
[documentation-url]: https://docs.rs/blis-sys2

