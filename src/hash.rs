use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash(pass: String) -> String {
    let mut hasher = Sha256::new();

    hasher.input_str(&pass);

    hasher.result_str()
}
