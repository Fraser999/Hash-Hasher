#![warn(unused, missing_copy_implementations)]
#![deny(
    deprecated_in_future,
    future_incompatible,
    macro_use_extern_crate,
    rust_2018_idioms,
    nonstandard_style,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    warnings,
    clippy::all,
    clippy::pedantic
)]
#![forbid(
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    incoherent_fundamental_impls,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]

mod utils;

use criterion::{criterion_group, criterion_main, Criterion};
use utils::{
    sha1::Digest as Sha1Digest, sha256::Digest as Sha256Digest, sha512::Digest as Sha512Digest,
    sip::Digest as SipDigest,
};

fn sha1_benchmarks(criterion: &mut Criterion) {
    let _ = criterion
        .bench_function("Hash SHA1s using default Hasher", |bencher| {
            utils::hash_using_default_hasher::<Sha1Digest>(bencher)
        })
        .bench_function("Hash SHA1s using HashHasher", |bencher| {
            utils::hash_using_hash_hasher::<Sha1Digest>(bencher)
        })
        .bench_function("Hash SHA1s using FnvHasher", |bencher| {
            utils::hash_using_fnv_hasher::<Sha1Digest>(bencher)
        })
        .bench_function("Insert SHA1s into set using default Hasher", |bencher| {
            utils::set_using_default_hasher::<Sha1Digest>(bencher)
        })
        .bench_function("Insert SHA1s into set using HashHasher", |bencher| {
            utils::set_using_hash_hasher::<Sha1Digest>(bencher)
        })
        .bench_function("Insert SHA1s_into_fnv_set", |bencher| {
            utils::set_using_fnv_hasher::<Sha1Digest>(bencher)
        });
}

fn sha256_benchmarks(criterion: &mut Criterion) {
    let _ = criterion
        .bench_function("Hash SHA256s using default Hasher", |bencher| {
            utils::hash_using_default_hasher::<Sha256Digest>(bencher)
        })
        .bench_function("Hash SHA256s using HashHasher", |bencher| {
            utils::hash_using_hash_hasher::<Sha256Digest>(bencher)
        })
        .bench_function("Hash SHA256s using FnvHasher", |bencher| {
            utils::hash_using_fnv_hasher::<Sha256Digest>(bencher)
        })
        .bench_function("Insert SHA256s into set using default Hasher", |bencher| {
            utils::set_using_default_hasher::<Sha256Digest>(bencher)
        })
        .bench_function("Insert SHA256s into set using HashHasher", |bencher| {
            utils::set_using_hash_hasher::<Sha256Digest>(bencher)
        })
        .bench_function("Insert SHA256s into set using FnvHasher", |bencher| {
            utils::set_using_fnv_hasher::<Sha256Digest>(bencher)
        });
}

fn sha512_benchmarks(criterion: &mut Criterion) {
    let _ = criterion
        .bench_function("Hash SHA512s using default Hasher", |bencher| {
            utils::hash_using_default_hasher::<Sha512Digest>(bencher)
        })
        .bench_function("Hash SHA512s using HashHasher", |bencher| {
            utils::hash_using_hash_hasher::<Sha512Digest>(bencher)
        })
        .bench_function("Hash SHA512s using FnvHasher", |bencher| {
            utils::hash_using_fnv_hasher::<Sha512Digest>(bencher)
        })
        .bench_function("Insert SHA512s into set using default Hasher", |bencher| {
            utils::set_using_default_hasher::<Sha512Digest>(bencher)
        })
        .bench_function("Insert SHA512s into set using HashHasher", |bencher| {
            utils::set_using_hash_hasher::<Sha512Digest>(bencher)
        })
        .bench_function("Insert SHA512s into set using FnvHasher", |bencher| {
            utils::set_using_fnv_hasher::<Sha512Digest>(bencher)
        });
}

fn sip_hash_benchmarks(criterion: &mut Criterion) {
    let _ = criterion
        .bench_function("Hash SIP hashes using default Hasher", |bencher| {
            utils::hash_using_default_hasher::<SipDigest>(bencher)
        })
        .bench_function("Hash SIP hashes using HashHasher", |bencher| {
            utils::hash_using_hash_hasher::<SipDigest>(bencher)
        })
        .bench_function("Hash SIP hashes using FnvHasher", |bencher| {
            utils::hash_using_fnv_hasher::<SipDigest>(bencher)
        })
        .bench_function(
            "Insert SIP hashes into set using default Hasher",
            |bencher| utils::set_using_default_hasher::<SipDigest>(bencher),
        )
        .bench_function("Insert SIP hashes into set using HashHasher", |bencher| {
            utils::set_using_hash_hasher::<SipDigest>(bencher)
        })
        .bench_function("Insert SIP hashes into set using FnvHasher", |bencher| {
            utils::set_using_fnv_hasher::<SipDigest>(bencher)
        });
}

criterion_group!(
    benches,
    sha1_benchmarks,
    sha256_benchmarks,
    sha512_benchmarks,
    sip_hash_benchmarks
);
criterion_main!(benches);
