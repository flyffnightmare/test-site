use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub title: String,
    pub short_description: String,
    pub full_description: String,
    pub image_url: String,
    pub screenshots: Vec<String>,
    pub genre: String,
    pub platform: String,  // Добавлено новое поле
    pub steam_url: Option<String>,
    pub release_date: Option<String>,
    pub developer: String,
    pub publisher: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub captcha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String, // Добавляем поле роли
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameListItem {
    pub id: Uuid,
    pub title: String,
    pub short_description: String,
    pub image_url: String,
    pub genre: String,
    pub platform: String,  // Добавлено для списка
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewsRequest {
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNewsRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: String, // "admin", "user"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subject: String,
    pub message: String,
    pub status: String, // "open", "in_progress", "resolved"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupportRequest {
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,        // user_id
    pub username: String,
    pub role: String,
    pub exp: usize,       // expiration
}