#![feature(test)]

extern crate test;

mod utils;

use test::Bencher;
use utils::{
    sha1::Digest as Sha1Digest, sha256::Digest as Sha256Digest, sha512::Digest as Sha512Digest,
    sip::Digest as SipDigest,
};

#[bench]
fn hash_sha1s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha1s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha1s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha1Digest>(bencher)
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
fn insert_sha1s_into_fnv_set(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha1Digest>(bencher)
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
fn hash_sha256s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha256Digest>(bencher)
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
fn insert_sha256s_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha256Digest>(bencher)
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
fn hash_sha512s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha512Digest>(bencher)
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
fn insert_sha512s_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha512Digest>(bencher)
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
fn hash_sip_hashes_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<SipDigest>(bencher)
}
