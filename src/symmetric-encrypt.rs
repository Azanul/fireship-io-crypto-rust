use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc;
use scrypt::password_hash::rand_core::{OsRng, RngCore};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const MESSAGE: &[u8] = b"i like turtles";

pub fn encrypt() {
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);

    let cipher = Aes256CbcEnc::new(&key.into(), &iv.into());

    let mut encrypted_message = [0u8; 48];
    let ct = cipher
        .encrypt_padded_b2b_mut::<Pkcs7>(MESSAGE, &mut encrypted_message)
        .unwrap();

    let decipher = Aes256CbcDec::new(&key.into(), &iv.into());

    let mut decrypted_message = [0u8; 48];
    match decipher.decrypt_padded_b2b_mut::<Pkcs7>(ct, &mut decrypted_message) {
        Ok(n) => println!("{:?}", hex::encode(n)),
        Err(err) => println!("Error: {}", err),
    };

    for d in decrypted_message {
        print!("{}", d as char);
    }
    println!();
}
