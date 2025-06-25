# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.4] - 2025-06-05

### Change

- Minor update to docs.
- Changed CI to use GitHub actions.
- Update dev dependencies `rand` and `fnv`.
- Update lints.
- General housekeeping.

## [2.0.3] - 2020-02-28

### Change

- Remove renamed lint.

## [2.0.2] - 2019-12-11

### Change

- Remove deprecated lints.

## [2.0.1] - 2019-12-11

### Change

- Remove deprecated lint.

### Fixed

- Fix broken doc links.

## [2.0.0] - 2019-04-28

### Changed

- Specify 2018 Edition in manifest.
- Update dev dependency `rand`.
- Update lints.

### Fixed

- Fixed [issue #1](https://github.com/Fraser999/Hash-Hasher/issues/1): result tending towards max_value with multiple calls to write.

## [1.1.0] - 2019-04-10

### Changed

- Rename aliases to `hash_hasher::HashedSet` and `hash_hasher::HashedMap`.
- Include FNV hasher in benchmarks.

## [1.0.0] - 2018-07-07

### Added

- Add alias for `hash_hasher::HashSet` and `hash_hasher::HashMap`.

### Changed

- Update dev dependency `rand`.
- Update CI scripts.
- Addressed new clippy warnings.

## [0.3.0] - 2017-06-11 (YANKED)

### Changed

- Minor updates (stylistic/formatting changes, improved README contents)

## [0.2.0] - 2016-05-31 (YANKED)

### Changed

- Reduce chance of hash-collisions in expected normal use-case by reversing the order of the hash's bytes.

## [0.1.0] - 2016-05-19 (YANKED)

### Added

- Initial implementation.

[2.0.4]: https://github.com/Fraser999/Hash-Hasher/compare/v2.0.3...v2.0.4
[2.0.3]: https://github.com/Fraser999/Hash-Hasher/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/Fraser999/Hash-Hasher/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/Fraser999/Hash-Hasher/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/Fraser999/Hash-Hasher/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/Fraser999/Hash-Hasher/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/Fraser999/Hash-Hasher/compare/v0.3.0...v1.0.0
[0.3.0]: https://github.com/Fraser999/Hash-Hasher/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Fraser999/Hash-Hasher/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Fraser999/Hash-Hasher/commit/6873ae4
