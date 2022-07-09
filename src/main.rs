pub mod argon2;
#[path = "../src/asymmetric-encrypt.rs"]
pub mod asymmetric_encrypt;
pub mod hash;
pub mod hmac;
pub mod salt;
pub mod sign;
#[path = "../src/symmetric-encrypt.rs"]
pub mod symmetric_encrypt;

extern crate hex;

fn main() {
    // let mut line = String::new();

    // println!("Enter a string :");

    // let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    // println!("{}", hash::hash(line.clone()));

    // println!("{}", salt::signup(format!("{}@email.com", line.trim_end()), line.clone()));
    // println!("{}", salt::login(format!("{}@email.com", line.trim_end()), line.clone()));

    // hmac::hmac();

    // symmetric_encrypt::encrypt();

    // asymmetric_encrypt::encrypt();

    sign::sign();
}
