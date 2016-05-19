use std::hash::{Hash, Hasher};

use rand::{Rand, Rng};

const DIGEST_SIZE: usize = 20;

#[derive(Copy, Clone, Eq)]
pub struct Digest(pub [u8; DIGEST_SIZE]);

impl PartialEq for Digest {
    fn eq(&self, other: &Digest) -> bool {
        &self.0[..] == &other.0[..]
    }
}

impl Hash for Digest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0[..])
    }
}

impl Rand for Digest {
    fn rand<R: Rng>(rng: &mut R) -> Digest {
        let mut digest = [0u8; DIGEST_SIZE];
        for c in digest[..].iter_mut() {
            *c = rng.gen();
        }
        Digest(digest)
    }
}
