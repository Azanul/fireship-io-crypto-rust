use crypto::mac::Mac;
use crypto::sha2::Sha256;
use crypto::hmac;

const KEY: &[u8] = b"super-secret!";
const MESSAGE: &[u8] = b"boo :ghost:";
const KEY2: &[u8] = b"other-password";

pub fn hmac() {
    let mut hmac = hmac::Hmac::new(Sha256::new(), KEY);
    hmac.input(MESSAGE);

    println!("{:?}", hex::encode(hmac.result().code()));

    let mut hmac2 = hmac::Hmac::new(Sha256::new(), KEY2);
    hmac2.input(MESSAGE);

    println!("{:?}", hex::encode(hmac2.result().code()));

}
