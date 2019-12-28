use sha2::{Digest, Sha256};

#[inline]
pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.input(input);
    hasher.result().into()
}
