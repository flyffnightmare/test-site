// auth.rs
use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::env;

// Функции для работы с паролями
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

// Функции валидации
pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();
    username_regex.is_match(username)
}

pub fn validate_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    
    has_upper && has_lower && has_digit
}

// Функции для работы с JWT
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

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        eprintln!("⚠️ JWT_SECRET not set, using fallback");
        "fallback-secret-key-for-development".to_string()
    });
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &str) -> Result<crate::models::Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        eprintln!("⚠️ JWT_SECRET not set, using fallback");
        "fallback-secret-key-for-development".to_string()
    });
    
    decode::<crate::models::Claims>(
        token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    ).map(|data| data.claims)
}

// Дополнительные функции валидации
pub fn validate_captcha(captcha: &str) -> bool {
    // Простая проверка капчи - в реальном приложении нужно усложнить
    captcha.len() == 4 && captcha.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn sanitize_input(input: &str) -> String {
    input.trim().to_string()
}