use argh::FromArgs;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};

const HASH: &str = "$argon2id$v=19$m=1048576,t=5,p=4$jYbMJN8k4lxVpmV+YH4Tsw$3YDYvjLCaaLCWvVnG01dXcXvwgWIhgWwzVpYcVx6Kok";

#[derive(FromArgs)]
/// Small tool for identity verification.
struct Input {
    /// input string to be verified
    #[argh(option)]
    input: String,
}

fn main() {
    let input: Input = argh::from_env();
    let parsed_hash = PasswordHash::new(&HASH).unwrap();
    match Argon2::default().verify_password(&input.input.into_bytes(), &parsed_hash) {
        Ok(_) => println!("Yup, that's me."),
        Err(_) => println!("Nope, that's not me."),
    }
}
