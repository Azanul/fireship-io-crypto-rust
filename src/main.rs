pub mod hash;
pub mod salt;
pub mod hmac;
// #[path="../src/symmetric-encrypt.rs"]
// pub mod symmetric_encrypt;

extern crate hex;


fn main() {
    let mut line = String::new();
   
    println!("Enter a string :");
   
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    println!("{}", hash::hash(line.clone()));

    println!("{}", salt::signup(format!("{}@email.com", line.trim_end()), line.clone()));
    println!("{}", salt::login(format!("{}@email.com", line.trim_end()), line.clone()));

    hmac::hmac();
}
