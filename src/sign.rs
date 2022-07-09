#[path = "../src/keypair.rs"]
mod keypair;

use rsa::{Hash::SHA2_256, PaddingScheme::PKCS1v15Sign, PublicKey};
use sha2::{Digest, Sha256};

pub fn sign() {
    let (public_key, private_key) = keypair::generate();

    let message = b"this data must be signed";

    let mut hasher = Sha256::new();
    hasher.update(&message);
    let hashed = hasher.finalize();

    let signed = private_key.sign(PKCS1v15Sign {hash: Some(SHA2_256)}, &hashed).unwrap();
    println!("{}", hex::encode(&signed));

    println!("{}", public_key.verify(PKCS1v15Sign {hash: Some(SHA2_256)}, &hashed, &signed).is_ok());
}
