//! A `std::hash::Hasher` which is designed to work with already-hashed data.
//!
//! # Examples
//!
//! ```
//! let hash_builder = HashBuildHasher::default();
//! let mut map = HashMap::with_hasher(hash_builder);
//! assert!(map.insert(1024, "1024").is_none()););
//! ```


#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences)]

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
        println!("msg is {:?}", msg);
        for i in 0..cmp::min(8, msg.len()) {
            self.value |= (msg[i] as u64) << i * 8;
        }
        println!("value is {}", self.value);
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
        hash_hasher.write(&[0, 4]);
        assert_eq!(1033, hash_hasher.finish());
        hash_hasher.write(&[0, 4, 1]);
        assert_eq!(66569, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[231, 3]);
        assert_eq!(999, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[255, 255, 255, 255, 0, 0, 0, 0]);
        assert_eq!(4294967295, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[255, 255, 255, 255, 255, 255, 255, 255, 0]);
        assert_eq!(18446744073709551615, hash_hasher.finish());

        hash_hasher = HashHasher::new();
        hash_hasher.write(&[1, 255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(18446744073709551361, hash_hasher.finish());
    }

    #[test]
    fn hash_map() {
        let hash_builder = HashBuildHasher::default();
        let mut map = HashMap::with_hasher(hash_builder);
        println!("Inserting 2:                    {:?}\n",
                 map.insert(2u64, "Two"));
        println!("Inserting 1:                    {:?}\n",
                 map.insert(1, "One"));
        println!("Inserting 2:                    {:?}\n",
                 map.insert(2u64, "Three"));
        println!("Inserting 1024:                 {:?}\n",
                 map.insert(1024, "1024"));
        println!("Inserting 999:                  {:?}\n",
                 map.insert(999, "999"));
        println!("Inserting 18446744073709551615: {:?}\n",
                 map.insert(18446744073709551615, "18446744073709551615"));
        println!("Inserting 4294967295:           {:?}\n",
                 map.insert(4294967295, "4294967295"));
        println!("{:?}", map);
    }
}
