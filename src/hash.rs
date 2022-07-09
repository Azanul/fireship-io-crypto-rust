use sha2::{Digest, Sha256};

pub fn hash(pass: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(&pass);

    hex::encode(hasher.finalize())
}
