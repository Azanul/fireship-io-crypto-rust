use rand_core::OsRng;
use rsa::{pkcs8::EncodePrivateKey, pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};

pub fn generate() -> (RsaPublicKey, RsaPrivateKey) {
    let mut rng = OsRng;

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    println!(
        "{}",
        public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap()
    );
    println!(
        "{:?}",
        private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap()
    );

    (public_key, private_key)
}
