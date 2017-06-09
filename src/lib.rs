//! A `std::hash::Hasher` which is designed to work with already-hashed or hash-like data.
//!
//! The provided hasher does minimal work under the assumption that the input data is already
//! suitable for use as a key in a `HashSet` or `HashMap`.
//!
//! As well as the performance benefit, it also causes `HashSet`s or `HashMap`s to become somewhat
//! deterministic.  Given two equal `HashSet`s or `HashMap`s containing more than a single element,
//! iterating them will yield the elements in differing orders.  By using a
//! `HashSet<_, HashBuildHasher>` or `HashMap<_, _, HashBuildHasher>`, then if the same data is
//! inserted and/or removed *in the same order*, iterating the collection will yield a consistent
//! order.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! use hash_hasher::HashBuildHasher;
//!
//! let hash_builder = HashBuildHasher::default();
//! let mut map: HashMap<u16, &str, HashBuildHasher> = HashMap::with_hasher(hash_builder);
//!
//! assert!(map.insert(0, "zero").is_none());
//! assert!(map.insert(1024, "1024").is_none());
//! assert_eq!(Some("zero"), map.insert(0, "nothing"));
//! ```

#![forbid(warnings)]
#![warn(missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results,
        variant_size_differences)]
#![cfg_attr(feature="cargo-clippy", deny(clippy, clippy_pedantic))]

use std::hash::{BuildHasherDefault, Hasher};

/// A hasher which does minimal work to create the required `u64` output under the assumption that
/// the input is already a hash digest or otherwise already suitable for use as a key in a `HashSet`
/// or `HashMap`.
#[derive(Copy, Clone)]
pub struct HashHasher {
    /// Holds internal state of hasher.
    value: u64,
}

impl Hasher for HashHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        // A normal use-case (e.g. by a node in a DHT) may well involve handling hashes which are
        // identical over the most-significant-bits, hence reverse the input message here to use the
        // least-significant-bits first.
        for (index, item) in msg.iter().rev().enumerate().take(8) {
            self.value |= (*item as u64) << (index * 8);
        }
    }

    #[inline]
    fn finish(&self) -> u64 { self.value }
}

impl Default for HashHasher {
    #[inline]
    fn default() -> HashHasher { HashHasher { value: 0 } }
}

/// Alias for a `BuildHasherDefault<HashHasher>`.
pub type HashBuildHasher = BuildHasherDefault<HashHasher>;



#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::{Rng, thread_rng};
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::hash::Hasher;

    #[test]
    fn hasher() {
        let mut hash_hasher = HashHasher::default();
        hash_hasher.write(&[9]);
        assert_eq!(9, hash_hasher.finish());
        hash_hasher.write(&[4, 0]);
        assert_eq!(1033, hash_hasher.finish());
        hash_hasher.write(&[1, 4, 0]);
        assert_eq!(66569, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[3, 231]);
        assert_eq!(999, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[0, 0, 0, 0, 255, 255, 255, 255]);
        assert_eq!(4294967295, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(18446744073709551361, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[0, 255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::default();
        hash_hasher.write(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());
    }

    #[test]
    fn hash_map() {
        let hash_builder = HashBuildHasher::default();
        let mut map = HashMap::with_hasher(hash_builder);
        let mut sha1 = [0_u8; 20];
        assert!(map.insert(sha1, "First").is_none());
        sha1[19] = 1;
        assert!(map.insert(sha1, "Second").is_none());
        sha1[0] = 1;
        assert!(map.insert(sha1, "Third").is_none());
        assert_eq!(map.insert(sha1, "Fourth"), Some("Third"));
    }

    #[test]
    fn determinism() {
        let mut set1 = HashSet::with_hasher(HashBuildHasher::default());
        let mut set2 = HashSet::with_hasher(HashBuildHasher::default());

        let mut set3 = HashSet::new();
        let mut set4 = HashSet::new();

        let mut generator = thread_rng();
        for _ in 0..100 {
            let random_value = generator.next_u64();
            let _ = set1.insert(random_value);
            let _ = set2.insert(random_value);
            let _ = set3.insert(random_value);
            let _ = set4.insert(random_value);
        }

        assert_eq!(set1.into_iter().collect::<Vec<_>>(), set2.into_iter().collect::<Vec<_>>());
        assert_ne!(set3.into_iter().collect::<Vec<_>>(), set4.into_iter().collect::<Vec<_>>());
    }
}
