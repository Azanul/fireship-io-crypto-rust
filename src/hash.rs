use sha2::{Sha256, Digest};

pub fn hash(pass: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(&pass);

    hex::encode(hasher.finalize())
}
