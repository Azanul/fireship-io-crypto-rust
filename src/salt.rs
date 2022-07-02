use std::fs;
use serde::{Serialize, Deserialize};

use scrypt::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    scrypt,
    Params
};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    password: String
}

pub fn signup(email: String, password: String) -> String{
    let params = Params::new(8, 8, 2).unwrap();

    let salt = SaltString::generate(&mut OsRng);

    let mut output = [0u8; 32];
    scrypt(password.as_bytes(), salt.as_bytes(), &params, &mut output).unwrap();

    let enc_pass = hex::encode(&output);

    let mut old_users: Vec<User> = serde_json::from_str(&fs::read_to_string("users.json").expect("Unable to read file")).unwrap();
    let new_user = User {email, password: format!("{}:{}", hex::encode(salt.as_bytes()), enc_pass)};
    old_users.push(new_user);

    fs::write("users.json", serde_json::to_string_pretty(&old_users).unwrap()).expect("Unable to write file");

    enc_pass
}

pub fn login(email: String, password: String) -> String {
    let users: Vec<User> = serde_json::from_str(&fs::read_to_string("users.json").expect("Unable to read file")).unwrap();
    
    let email_match_user = match users.into_iter().find(|user| user.email == email).ok_or("Invalid Email") {
        Ok(user) => user,
        Err(e) => return e.to_string()
    };

    let mut split_pass = email_match_user.password.split(':');
    let (salt, pass) = (split_pass.next().unwrap(), split_pass.next().unwrap());
    
    let params = Params::new(8, 8, 2).unwrap();
    let mut output = [0u8; 32];
    scrypt(password.as_bytes(), &hex::decode(salt).unwrap(), &params, &mut output).unwrap();

    if hex::encode(&output) != pass {
        return "Invalid Password".to_string()
    }
    "Logged In".to_string()
}