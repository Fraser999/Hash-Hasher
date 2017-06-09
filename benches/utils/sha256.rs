use rand::{Rand, Rng};
use std::hash::{Hash, Hasher};

const DIGEST_SIZE: usize = 32;

#[derive(Copy, Clone, Eq)]
pub struct Digest(pub [u8; DIGEST_SIZE]);

impl PartialEq for Digest {
    fn eq(&self, other: &Digest) -> bool { self.0[..] == other.0[..] }
}

impl Hash for Digest {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(&self.0[..]) }
}

impl Rand for Digest {
    fn rand<R: Rng>(rng: &mut R) -> Digest {
        let mut digest = [0_u8; DIGEST_SIZE];
        for c in digest[..].iter_mut() {
            *c = rng.gen();
        }
        Digest(digest)
    }
}
