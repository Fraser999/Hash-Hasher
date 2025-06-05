# hash_hasher

A [`std::hash::Hasher`](https://doc.rust-lang.org/std/hash/trait.Hasher.html)
which is designed to work with already-hashed or hash-like data.

[![Documentation](https://docs.rs/hash_hasher/badge.svg)](https://docs.rs/hash_hasher)
[![Crates.io](https://img.shields.io/crates/v/hash_hasher.svg)](https://crates.io/crates/hash_hasher)
[![Build Status](https://github.com/Fraser999/Hash-Hasher/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Fraser999/Hash-Hasher/actions/workflows/ci.yml)

## Details

The provided hasher does minimal work under the assumption that the input data
is already suitable for use as a key in a `HashSet` or `HashMap`.

As well as the performance benefit, it also causes `HashSet`s or `HashMap`s to
become somewhat deterministic. Given two equal `HashSet`s or `HashMap`s
containing more than a single element, iterating them will likely yield the
elements in differing orders. By using a
[`hash_hasher::HashedSet`](https://docs.rs/hash_hasher/*/hash_hasher/type.HashedSet.html)
or
[`hash_hasher::HashedMap`](https://docs.rs/hash_hasher/*/hash_hasher/type.HashedMap.html),
then if the same data is inserted and/or removed *in the same order*, iterating
the collection will yield a consistent order.

## Example

Since `new()` and `with_capacity()` aren't available for `HashSet`s or
`HashMap`s using custom hashers, the available constructors are `default()`,
`with_hasher()` and `with_capacity_and_hasher()`.

```rust
use hash_hasher::{HashBuildHasher, HashedMap, HashedSet};

let mut map = HashedMap::default ();
assert!(map.insert(0, "zero").is_none());

let mut set = HashedSet::with_capacity_and_hasher(100, HashBuildHasher::default ());
assert!(set.insert(0));
```

## Benchmarks

A benchmark suite is included and sample figures can be found in the
[CI results](https://github.com/Fraser999/Hash-Hasher/actions/workflows/ci.yml)
in the `benchmarks-and-checks` job under the `Run benchmarks` step.

For example:

```
test hash_sha1s_using_default_hasher                 ... bench:         191.12 ns/iter (+/- 17.41)
test hash_sha1s_using_fnv_hasher                     ... bench:         196.12 ns/iter (+/- 3.09)
test hash_sha1s_using_hash_hasher                    ... bench:           0.18 ns/iter (+/- 0.00)
test hash_sha256s_using_default_hasher               ... bench:         248.04 ns/iter (+/- 4.94)
test hash_sha256s_using_fnv_hasher                   ... bench:         369.20 ns/iter (+/- 3.10)
test hash_sha256s_using_hash_hasher                  ... bench:           0.18 ns/iter (+/- 0.00)
test hash_sha512s_using_default_hasher               ... bench:         263.13 ns/iter (+/- 0.30)
test hash_sha512s_using_fnv_hasher                   ... bench:       1,058.13 ns/iter (+/- 3.17)
test hash_sha512s_using_hash_hasher                  ... bench:           0.18 ns/iter (+/- 0.01)
test hash_sip_hashes_using_default_hasher            ... bench:           0.18 ns/iter (+/- 0.01)
test hash_sip_hashes_using_fnv_hasher                ... bench:           0.18 ns/iter (+/- 0.01)
test hash_sip_hashes_using_hash_hasher               ... bench:           0.18 ns/iter (+/- 0.01)
test insert_sha1s_into_fnv_set                       ... bench:         281.13 ns/iter (+/- 5.70)
test insert_sha1s_into_set_using_default_hasher      ... bench:         381.28 ns/iter (+/- 9.83)
test insert_sha1s_into_set_using_hash_hasher         ... bench:          97.50 ns/iter (+/- 0.69)
test insert_sha256s_into_set_using_default_hasher    ... bench:         407.91 ns/iter (+/- 20.46)
test insert_sha256s_into_set_using_fnv_hasher        ... bench:         823.43 ns/iter (+/- 41.30)
test insert_sha256s_into_set_using_hash_hasher       ... bench:          90.92 ns/iter (+/- 0.89)
test insert_sha512s_into_set_using_default_hasher    ... bench:         541.99 ns/iter (+/- 18.94)
test insert_sha512s_into_set_using_fnv_hasher        ... bench:       1,338.65 ns/iter (+/- 47.01)
test insert_sha512s_into_set_using_hash_hasher       ... bench:         190.27 ns/iter (+/- 1.28)
test insert_sip_hashes_into_set_using_default_hasher ... bench:         265.87 ns/iter (+/- 3.10)
test insert_sip_hashes_into_set_using_fnv_hasher     ... bench:         113.35 ns/iter (+/- 4.27)
test insert_sip_hashes_into_set_using_hash_hasher    ... bench:          55.40 ns/iter (+/- 0.16)
```

## License

Licensed under either of

* [Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0) (see
  also [LICENSE-APACHE](LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT) (see
  also [LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
