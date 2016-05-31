//! A `std::hash::Hasher` which is designed to work with already-hashed data.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! use hash_hasher::HashBuildHasher;
//!
//! let hash_builder = HashBuildHasher::default();
//! let mut map = HashMap::with_hasher(hash_builder);
//!
//! assert!(map.insert(0, "zero").is_none());
//! assert!(map.insert(1024, "1024").is_none());
//! assert_eq!(Some("zero"), map.insert(0, "nothing"));
//! ```

#![deny(missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unused_extern_crates,
        unused_import_braces, unused_qualifications, variant_size_differences, warnings)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]

use std::cmp;
use std::hash::{BuildHasherDefault, Hasher};

/// A hasher which does minimal work to create the required `u64` output under the assumption that
/// the input is already a hash digest or otherwise already suitable for use as a key in a `HashSet`
/// or `HashMap`.
pub struct HashHasher {
    value: u64,
}

impl HashHasher {
    /// Creates a new `HashHasher`.
    #[inline]
    pub fn new() -> HashHasher {
        HashHasher { value: 0 }
    }
}

impl Hasher for HashHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        let byte_count = cmp::min(8, msg.len());
        // A normal use-case (e.g. by a node in a DHT) may well involve handling hashes which are
        // identical over the most-significant-bits, hence reverse the input message here to use the
        // least-significant-bits first.
        for (index, item) in msg.iter().rev().enumerate().take(byte_count) {
            self.value |= (*item as u64) << (index * 8);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.value
    }
}

impl Clone for HashHasher {
    #[inline]
    fn clone(&self) -> HashHasher {
        HashHasher { value: self.value }
    }
}

impl Default for HashHasher {
    fn default() -> HashHasher {
        HashHasher::new()
    }
}

/// Alias for a `BuildHasherDefault<HashHasher>`.
pub type HashBuildHasher = BuildHasherDefault<HashHasher>;



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hasher;

    #[test]
    fn hasher() {
        let mut hash_hasher = HashHasher::new();
        hash_hasher.write(&[9]);
        assert_eq!(9, hash_hasher.finish());
        hash_hasher.write(&[4, 0]);
        assert_eq!(1033, hash_hasher.finish());
        hash_hasher.write(&[1, 4, 0]);
        assert_eq!(66569, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[3, 231]);
        assert_eq!(999, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[0, 0, 0, 0, 255, 255, 255, 255]);
        assert_eq!(4294967295, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(18446744073709551361, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[0, 255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());
    }

    #[test]
    fn hash_map() {
        let hash_builder = HashBuildHasher::default();
        let mut map = HashMap::with_hasher(hash_builder);
        let mut sha1 = [0u8; 20];
        assert!(map.insert(sha1, "First").is_none());
        sha1[19] = 1;
        assert!(map.insert(sha1, "Second").is_none());
        sha1[0] = 1;
        assert!(map.insert(sha1, "Third").is_none());
        assert_eq!(map.insert(sha1, "Fourth"), Some("Third"));
    }
}
