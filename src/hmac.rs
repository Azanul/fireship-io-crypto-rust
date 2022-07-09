use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const KEY: &[u8] = b"super-secret!";
const MESSAGE: &[u8] = b"boo :ghost:";
const KEY2: &[u8] = b"other-password";

pub fn hmac() {
    let mut hmac = HmacSha256::new_from_slice(KEY).expect("Expectations");
    hmac.update(MESSAGE);

    println!("{:?}", hex::encode(hmac.finalize().into_bytes()));

    let mut hmac2 = HmacSha256::new_from_slice(KEY2).expect("Expectations");
    hmac2.update(MESSAGE);

    println!("{:?}", hex::encode(hmac2.finalize().into_bytes()));
}
