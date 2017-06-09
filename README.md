# hash_hasher

A `std::hash::Hasher` which is designed to work with already-hashed or hash-like data.

[![Documentation](https://docs.rs/hash_hasher/badge.svg)](https://docs.rs/hash_hasher)
[![](http://meritbadge.herokuapp.com/hash_hasher)](https://crates.io/crates/hash_hasher)
[![Build status](https://ci.appveyor.com/api/projects/status/qnh6484hh5hlsh8a/branch/master?svg=true)](https://ci.appveyor.com/project/Fraser999/hash-hasher/branch/master)
[![Build Status](https://travis-ci.org/Fraser999/Hash-Hasher.svg?branch=master)](https://travis-ci.org/Fraser999/Hash-Hasher)

## Details

The provided hasher does minimal work under the assumption that the input data is already suitable
for use as a key in a `HashSet` or `HashMap`.

As well as the performance benefit, it also causes `HashSet`s or `HashMap`s to become somewhat
deterministic.  Given two equal `HashSet`s or `HashMap`s containing more than a single element,
iterating them will yield the elements in differing orders.  By using a `HashSet<_, HashBuildHasher>`
or `HashMap<_, _, HashBuildHasher>`, then if the same data is inserted and/or removed *in the same
order*, iterating the collection will yield a consistent order.

## Example

```rust
extern crate hash_hasher;

use std::collections::HashMap;
use hash_hasher::HashBuildHasher;

let hash_builder = HashBuildHasher::default();
let mut map: HashMap<u16, &str, HashBuildHasher> = HashMap::with_hasher(hash_builder);

assert!(map.insert(0, "zero").is_none());
assert!(map.insert(1024, "1024").is_none());
assert_eq!(Some("zero"), map.insert(0, "nothing"));
```

## Benchmarks

A benchmark suite is included and sample figures can be found at the end of the
[AppVeyor results](https://ci.appveyor.com/project/Fraser999/hash-hasher/branch/master) and the
nightly jobs of the [Travis results](https://travis-ci.org/Fraser999/Hash-Hasher).

For example:

```
insert_sha1s_into_set_using_default_hasher      ... bench:       3,382 ns/iter (+/- 276)
insert_sha1s_into_set_using_hash_hasher         ... bench:       1,657 ns/iter (+/- 166)

insert_sha256s_into_set_using_default_hasher    ... bench:       4,002 ns/iter (+/- 374)
insert_sha256s_into_set_using_hash_hasher       ... bench:       1,523 ns/iter (+/- 82)

insert_sha512s_into_set_using_default_hasher    ... bench:       5,128 ns/iter (+/- 228)
insert_sha512s_into_set_using_hash_hasher       ... bench:       1,859 ns/iter (+/- 109)

insert_sip_hashes_into_set_using_default_hasher ... bench:       2,351 ns/iter (+/- 171)
insert_sip_hashes_into_set_using_hash_hasher    ... bench:         630 ns/iter (+/- 13)
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
