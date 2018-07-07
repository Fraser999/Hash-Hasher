#![forbid(warnings)]
#![warn(
    missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code,
    unused_extern_crates, unused_import_braces, unused_qualifications, unused_results,
    variant_size_differences
)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![feature(test)]

extern crate hash_hasher;
extern crate rand;
extern crate test;

mod utils;

use test::Bencher;
use utils::sha1::Digest as Sha1Digest;
use utils::sha256::Digest as Sha256Digest;
use utils::sha512::Digest as Sha512Digest;
use utils::sip::Digest as SipDigest;

#[bench]
fn hash_sha1s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha1s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn insert_sha1s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn insert_sha1s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha256s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn hash_sha256s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn insert_sha256s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn insert_sha256s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn hash_sha512s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn hash_sha512s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn insert_sha512s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn insert_sha512s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn hash_sip_hashes_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<SipDigest>(bencher)
}

#[bench]
fn hash_sip_hashes_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<SipDigest>(bencher)
}
