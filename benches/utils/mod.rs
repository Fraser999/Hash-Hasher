pub mod sha1;
pub mod sha256;
pub mod sha512;
pub mod sip;

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash, Hasher, SipHasher};

use hash_hasher::{HashBuildHasher, HashHasher};
use rand::{self, Rand};
use test::Bencher;

const COUNT: usize = 100;

fn random_data<T: Rand>() -> Vec<T> {
    let mut data = Vec::<T>::with_capacity(COUNT);
    for _ in 0..COUNT {
        data.push(rand::random());
    }
    data
}

fn perform_hash<T: Hash + Rand, H: Hasher + Default>(bencher: &mut Bencher) {
    let data = random_data::<T>();
    bencher.iter(|| {
        for element in &data {
            let mut hasher = H::default();
            element.hash(&mut hasher);
            let _ = hasher.finish();
        }
    });
}

fn insert_to_set<T: Copy + Eq + Hash + Rand, S: BuildHasher>(set: &mut HashSet<T, S>,
                                                             bencher: &mut Bencher) {
    let data = random_data();
    bencher.iter(|| {
        for element in &data {
            let _ = set.insert(*element);
        }
    });
    assert!(!set.is_empty());
}

pub fn hash_using_default_hasher<T: Hash + Rand>(bencher: &mut Bencher) {
    perform_hash::<T, SipHasher>(bencher);
}

pub fn hash_using_hash_hasher<T: Hash + Rand>(bencher: &mut Bencher) {
    perform_hash::<T, HashHasher>(bencher);
}

pub fn set_using_default_hasher<T: Copy + Eq + Hash + Rand>(bencher: &mut Bencher) {
    let mut set = HashSet::<T>::with_capacity(COUNT);
    insert_to_set(&mut set, bencher);
}

pub fn set_using_hash_hasher<T: Copy + Eq + Hash + Rand>(bencher: &mut Bencher) {
    let hash_builder = HashBuildHasher::default();
    let mut set = HashSet::<T, HashBuildHasher>::with_capacity_and_hasher(COUNT, hash_builder);
    insert_to_set(&mut set, bencher);
}
