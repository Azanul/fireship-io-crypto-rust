use rand_core::OsRng;
use rsa::{PaddingScheme, PublicKey};

pub mod keypair;

pub fn encrypt() {
    let (public_key, private_key) = keypair::generate();

    let data = b"the british are coming";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key
        .encrypt(&mut OsRng, padding, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..], "Encryption Failed...");

    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key
        .decrypt(padding, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..], "Decryption Failed...");
    println!("{:?}", String::from_utf8(dec_data).unwrap());
}
