use serde::{Deserialize, Serialize};
use std::fs;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    password: String,
}

pub fn signup(email: String, password: String) -> String {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let mut old_users: Vec<User> =
        serde_json::from_str(&fs::read_to_string("users_argon.json").expect("Unable to read file"))
            .unwrap();
    let new_user = User { email, password };
    old_users.push(new_user);

    fs::write(
        "users_argon.json",
        serde_json::to_string_pretty(&old_users).unwrap(),
    )
    .expect("Unable to write file");

    password_hash
}

pub fn login(email: String, password: String) -> String {
    let argon2 = Argon2::default();

    let users: Vec<User> =
        serde_json::from_str(&fs::read_to_string("users_argon.json").expect("Unable to read file"))
            .unwrap();

    let email_match_user = match users
        .into_iter()
        .find(|user| user.email == email)
        .ok_or("Invalid Email")
    {
        Ok(user) => user,
        Err(e) => return e.to_string(),
    };

    let mut split_pass = email_match_user.password.split(':');
    let (salt, pass) = (split_pass.next().unwrap(), split_pass.next().unwrap());

    let password_hash = argon2
        .hash_password(pass.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return "Invalid Password".to_string();
    }
    "Logged In".to_string()
}
