use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::env;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();
    username_regex.is_match(username)
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 6
}

pub fn create_jwt(user_id: Uuid, username: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = crate::models::Claims {
        sub: user_id,
        username: username.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}