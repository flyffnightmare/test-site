use crate::auth::*;
use crate::models::*;
use actix_web::{web, HttpResponse};
use sqlx::{PgPool, Row};

pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Валидация входных данных
    if !validate_username(&user_data.username) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Неверный формат имени пользователя (3-20 символов, только буквы, цифры и подчеркивания)".to_string()),
        });
    }

    if !validate_email(&user_data.email) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Неверный формат email".to_string()),
        });
    }

    if !validate_password(&user_data.password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Пароль должен содержать минимум 6 символов".to_string()),
        });
    }

    // Проверяем, существует ли пользователь
    let existing_user = sqlx::query("SELECT id FROM users WHERE username = $1 OR email = $2")
        .bind(&user_data.username)
        .bind(&user_data.email)
        .fetch_optional(pool.get_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Пользователь с таким именем или email уже существует".to_string()),
            });
        }
        Err(e) => {
            eprintln!("❌ Ошибка базы данных при проверке пользователя: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка базы данных".to_string()),
            });
        }
        _ => {}
    }

    // Хешируем пароль
    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при создании пароля".to_string()),
            });
        }
    };

    // Создаем пользователя
    match sqlx::query(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email"
    )
    .bind(&user_data.username)
    .bind(&user_data.email)
    .bind(&password_hash)
    .fetch_one(pool.get_ref())
    .await {
        Ok(row) => {
            let user_response = UserResponse {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(user_response),
                message: Some("Пользователь успешно зарегистрирован".to_string()),
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при создании пользователя: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при создании пользователя".to_string()),
            })
        }
    }
}

pub async fn login(pool: web::Data<PgPool>, login_data: web::Json<LoginRequest>) -> HttpResponse {
    // Пропускаем проверку капчи для демо

    match sqlx::query("SELECT id, username, email, password_hash FROM users WHERE username = $1")
        .bind(&login_data.username)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(row)) => {
            let password_hash: String = row.get("password_hash");

            if verify_password(&login_data.password, &password_hash).unwrap_or(false) {
                let user_response = UserResponse {
                    id: row.get("id"),
                    username: row.get("username"),
                    email: row.get("email"),
                };

                let auth_response = AuthResponse {
                    token: "demo-jwt-token".to_string(),
                    user: user_response,
                };

                HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    data: Some(auth_response),
                    message: Some("Вход выполнен успешно".to_string()),
                })
            } else {
                HttpResponse::Unauthorized().json(ApiResponse::<()> {
                    success: false,
                    data: None,
                    message: Some("Неверный пароль".to_string()),
                })
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Пользователь не найден".to_string()),
        }),
        Err(e) => {
            eprintln!("❌ Ошибка базы данных при входе: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка базы данных".to_string()),
            })
        }
    }
}

pub async fn get_games(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT id, title, description, image_url, genre, price, created_at FROM games ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let games: Vec<Game> = rows.iter().map(|row| {
                Game {
                    id: row.get("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    image_url: row.get("image_url"),
                    genre: row.get("genre"),
                    price: row.get("price"),  // Теперь price: f32 (REAL в PostgreSQL)
                    created_at: row.get("created_at"),
                }
            }).collect();

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(games),
                message: None,
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при получении игр: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при получении списка игр".to_string()),
            })
        }
    }
}
