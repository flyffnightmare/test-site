use actix_web::{web, HttpResponse, HttpRequest, HttpMessage}; 
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::models::*;
use crate::auth::*;

// Публичные обработчики
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
            message: Some("Пароль должен содержать минимум 8 символов, включая заглавные, строчные буквы и цифры".to_string()),
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
            // Добавляем роль по умолчанию для нового пользователя
            let user_id: Uuid = row.get("id");
            sqlx::query("INSERT INTO user_roles (user_id, role) VALUES ($1, 'user')")
                .bind(user_id)
                .execute(pool.get_ref())
                .await
                .ok(); // Игнорируем ошибку если роль уже существует

            let user_response = UserResponse {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                role: "user".to_string(),
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

pub async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> HttpResponse {
    println!("🔐 Попытка входа пользователя: {}", login_data.username);
    
    match sqlx::query("SELECT id, username, email, password_hash FROM users WHERE username = $1")
        .bind(&login_data.username)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            let password_hash: String = row.get("password_hash");
            let user_id: Uuid = row.get("id");
            let username: String = row.get("username");
            
            if verify_password(&login_data.password, &password_hash).unwrap_or(false) {
                // Получаем роль пользователя
                let role_result = sqlx::query("SELECT role FROM user_roles WHERE user_id = $1")
                    .bind(user_id)
                    .fetch_optional(pool.get_ref())
                    .await;
                
                let role = match role_result {
                    Ok(Some(role_row)) => {
                        let role_str = role_row.get::<String, &str>("role");
                        println!("👤 Роль пользователя {}: {}", username, role_str);
                        role_str
                    }
                    _ => {
                        println!("⚠️ Роль не найдена, устанавливаем 'user'");
                        "user".to_string()
                    }
                };

                // Создаем JWT токен
                println!("🎫 Создание JWT для пользователя {} с ролью {}", username, role);
                let token = match create_jwt(user_id, &username, &role) {
                    Ok(token) => {
                        println!("✅ JWT успешно создан");
                        token
                    }
                    Err(e) => {
                        eprintln!("❌ Ошибка создания JWT токена: {}", e);
                        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            success: false,
                            data: None,
                            message: Some("Ошибка создания токена".to_string()),
                        });
                    }
                };

                let user_response = UserResponse {
                    id: user_id,
                    username: username.clone(),
                    email: row.get("email"),
                    role: role.clone(),
                };

                let auth_response = AuthResponse {
                    token: token,
                    user: user_response,
                };

                println!("✅ Успешный вход пользователя {}", username);
                HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    data: Some(auth_response),
                    message: Some("Вход выполнен успешно".to_string()),
                })
            } else {
                println!("❌ Неверный пароль для пользователя {}", login_data.username);
                HttpResponse::Unauthorized().json(ApiResponse::<()> {
                    success: false,
                    data: None,
                    message: Some("Неверный пароль".to_string()),
                })
            }
        }
        Ok(None) => {
            println!("❌ Пользователь {} не найден", login_data.username);
            HttpResponse::Unauthorized().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Пользователь не найден".to_string()),
            })
        }
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
    match sqlx::query("SELECT id, title, short_description, image_url, genre, platform, created_at FROM games ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await {
        Ok(rows) => {
            let games: Vec<GameListItem> = rows.iter().map(|row| {
                GameListItem {
                    id: row.get("id"),
                    title: row.get("title"),
                    short_description: row.get("short_description"),
                    image_url: row.get("image_url"),
                    genre: row.get("genre"),
                    platform: row.get("platform"),
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

pub async fn get_game(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let game_id = path.into_inner();
    
    println!("🔍 Получение игры с ID: {}", game_id);
    
    match sqlx::query("SELECT * FROM games WHERE id = $1")
        .bind(game_id)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            let screenshots: Vec<String> = row.get("screenshots");
            
            let game = Game {
                id: row.get("id"),
                title: row.get("title"),
                short_description: row.get("short_description"),
                full_description: row.get("full_description"),
                image_url: row.get("image_url"),
                screenshots: screenshots,
                genre: row.get("genre"),
                platform: row.get("platform"),
                steam_url: row.get("steam_url"),
                release_date: row.get("release_date"),
                developer: row.get("developer"),
                publisher: row.get("publisher"),
                created_at: row.get("created_at"),
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(game),
                message: None,
            })
        }
        Ok(None) => {
            println!("❌ Игра с ID {} не найдена", game_id);
            HttpResponse::NotFound().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Игра не найдена".to_string()),
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при получении игры: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при получении игры".to_string()),
            })
        }
    }
}

// Новостные обработчики
pub async fn get_news(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query(
        "SELECT n.*, u.username as author_name FROM news n 
         JOIN users u ON n.author_id = u.id 
         ORDER BY n.created_at DESC LIMIT 10"
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            let news: Vec<News> = rows.iter().map(|row| {
                News {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    image_url: row.get("image_url"),
                    author_id: row.get("author_id"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            }).collect();

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(news),
                message: None,
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при получении новостей: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при получении новостей".to_string()),
            })
        }
    }
}

pub async fn create_news(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    news_data: web::Json<CreateNewsRequest>,
) -> HttpResponse {
    // Получаем токен из заголовка
    let auth_header = req.headers().get("Authorization");
    
    let token = if let Some(header) = auth_header {
        if let Ok(header_str) = header.to_str() {
            if header_str.starts_with("Bearer ") {
                Some(&header_str[7..])
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let token = match token {
        Some(t) => t,
        None => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Токен не предоставлен".to_string()),
            });
        }
    };

    // Валидируем JWT токен
    let claims = match crate::auth::validate_jwt(token) {
        Ok(claims) => claims,
        Err(e) => {
            println!("❌ Ошибка валидации JWT: {}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Невалидный токен".to_string()),
            });
        }
    };

    // Проверяем права (только админы могут создавать новости)
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Недостаточно прав".to_string()),
        });
    }

    match sqlx::query(
        "INSERT INTO news (title, content, image_url, author_id) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, title, content, image_url, author_id, created_at, updated_at"
    )
    .bind(&news_data.title)
    .bind(&news_data.content)
    .bind(&news_data.image_url.as_ref())
    .bind(&claims.sub) // Используем ID из JWT токена
    .fetch_one(pool.get_ref())
    .await {
        Ok(row) => {
            let news = News {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                image_url: row.get("image_url"),
                author_id: row.get("author_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(news),
                message: Some("Новость успешно создана".to_string()),
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при создании новости: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при создании новости".to_string()),
            })
        }
    }
}

// Обработчики для поддержки
pub async fn create_support_request(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    request_data: web::Json<CreateSupportRequest>,
) -> HttpResponse {
    match sqlx::query(
        "INSERT INTO support_requests (user_id, subject, message) 
         VALUES ($1, $2, $3) 
         RETURNING id, user_id, subject, message, status, created_at, updated_at"
    )
    .bind(user_id.into_inner())
    .bind(&request_data.subject)
    .bind(&request_data.message)
    .fetch_one(pool.get_ref())
    .await {
        Ok(row) => {
            let request = SupportRequest {
                id: row.get("id"),
                user_id: row.get("user_id"),
                subject: row.get("subject"),
                message: row.get("message"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(request),
                message: Some("Запрос в поддержку отправлен".to_string()),
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при создании запроса в поддержку: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при отправке запроса".to_string()),
            })
        }
    }
}

// Health check
pub async fn health_check(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            data: None,
            message: Some("✅ Сервер и БД работают нормально".to_string()),
        }),
        Err(e) => {
            eprintln!("❌ Ошибка проверки здоровья БД: {}", e);
            HttpResponse::ServiceUnavailable().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("❌ Проблемы с подключением к БД".to_string()),
            })
        }
    }
}

// Получение текущего пользователя
// handlers.rs - в функции get_current_user
pub async fn get_current_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // Получаем токен из заголовка
    let auth_header = req.headers().get("Authorization");
    
    let token = if let Some(header) = auth_header {
        if let Ok(header_str) = header.to_str() {
            if header_str.starts_with("Bearer ") {
                Some(&header_str[7..]) // Убираем "Bearer "
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let token = match token {
        Some(t) => t,
        None => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Токен не предоставлен".to_string()),
            });
        }
    };

    // Валидируем JWT токен
    let claims = match crate::auth::validate_jwt(token) {
        Ok(claims) => claims,
        Err(e) => {
            println!("❌ Ошибка валидации JWT: {}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Невалидный токен".to_string()),
            });
        }
    };

    // Получаем пользователя из базы
    match sqlx::query("SELECT id, username, email FROM users WHERE id = $1")
        .bind(&claims.sub)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            let user_response = UserResponse {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                role: claims.role,
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(user_response),
                message: None,
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Пользователь не найден".to_string()),
        }),
        Err(e) => {
            eprintln!("❌ Ошибка при получении пользователя: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка базы данных".to_string()),
            })
        }
    }
}

// АДМИНСКИЕ ОБРАБОТЧИКИ

// Статистика для админки
pub async fn get_admin_stats(pool: web::Data<PgPool>) -> HttpResponse {
    let users_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let games_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM games")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let news_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let support_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM support_requests")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let open_support_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM support_requests WHERE status = 'open'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let stats = serde_json::json!({
        "users": users_count,
        "games": games_count,
        "news": news_count,
        "supportRequests": support_count,
        "openSupportRequests": open_support_count
    });

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(stats),
        message: None,
    })
}

// Получение списка пользователей
pub async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query(
        "SELECT u.id, u.username, u.email, u.created_at, 
                COALESCE(ur.role, 'user') as role
         FROM users u
         LEFT JOIN user_roles ur ON u.id = ur.user_id
         ORDER BY u.created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            let users: Vec<serde_json::Value> = rows.iter().map(|row| {
                serde_json::json!({
                    "id": row.get::<Uuid, &str>("id"),
                    "username": row.get::<String, &str>("username"),
                    "email": row.get::<String, &str>("email"),
                    "role": row.get::<String, &str>("role"),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, &str>("created_at")
                })
            }).collect();

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(users),
                message: None,
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при получении пользователей: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при получении пользователей".to_string()),
            })
        }
    }
}

// Создание пользователя (админ)
pub async fn create_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> HttpResponse {
    // Валидация
    if !validate_username(&user_data.username) || !validate_email(&user_data.email) || !validate_password(&user_data.password) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Неверные данные пользователя".to_string()),
        });
    }

    // Проверяем существование пользователя
    let existing_user = sqlx::query("SELECT id FROM users WHERE username = $1 OR email = $2")
        .bind(&user_data.username)
        .bind(&user_data.email)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(Some(_)) = existing_user {
        return HttpResponse::Conflict().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Пользователь с таким именем или email уже существует".to_string()),
        });
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
            let user_id: Uuid = row.get("id");
            
            // Добавляем роль пользователя
            let role = user_data.role.as_deref().unwrap_or("user");
            sqlx::query("INSERT INTO user_roles (user_id, role) VALUES ($1, $2)")
                .bind(user_id)
                .bind(role)
                .execute(pool.get_ref())
                .await
                .ok();

            let user_response = UserResponse {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                role: role.to_string(),
            };

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(user_response),
                message: Some("Пользователь успешно создан".to_string()),
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


// Обновление пользователя
pub async fn update_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    user_data: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // Проверяем существование пользователя
    let user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(None) = user_exists {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Пользователь не найден".to_string()),
        });
    }

    // Обновляем данные пользователя
    let mut query = "UPDATE users SET updated_at = NOW()".to_string();
    let mut params: Vec<String> = Vec::new();
    let mut counter = 1;

    if let Some(username) = &user_data.username {
        if !validate_username(username) {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Неверный формат имени пользователя".to_string()),
            });
        }
        query.push_str(&format!(", username = ${}", counter));
        params.push(username.clone());
        counter += 1;
    }

    if let Some(email) = &user_data.email {
        if !validate_email(email) {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Неверный формат email".to_string()),
            });
        }
        query.push_str(&format!(", email = ${}", counter));
        params.push(email.clone());
        counter += 1;
    }

    if let Some(password) = &user_data.password {
        if !validate_password(password) {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Неверный формат пароля".to_string()),
            });
        }
        let password_hash = match hash_password(password) {
            Ok(hash) => hash,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    success: false,
                    data: None,
                    message: Some("Ошибка при хешировании пароля".to_string()),
                });
            }
        };
        query.push_str(&format!(", password_hash = ${}", counter));
        params.push(password_hash);
        counter += 1;
    }

    query.push_str(" WHERE id = $");
    query.push_str(&counter.to_string());
    params.push(user_id.to_string());

    // Выполняем обновление
    let mut sql_query = sqlx::query(&query);
    for param in &params {
        sql_query = sql_query.bind(param);
    }

    match sql_query.execute(pool.get_ref()).await {
        Ok(_) => {
            // Обновляем роль если нужно
            if let Some(role) = &user_data.role {
                match sqlx::query(
                    "INSERT INTO user_roles (user_id, role) VALUES ($1, $2) 
                     ON CONFLICT (user_id) DO UPDATE SET role = $2"
                )
                .bind(user_id)
                .bind(role)
                .execute(pool.get_ref())
                .await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("❌ Ошибка при обновлении роли: {}", e);
                    }
                }
            }

            HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Пользователь успешно обновлен".to_string()),
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при обновлении пользователя: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при обновлении пользователя".to_string()),
            })
        }
    }
}

pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // Удаляем связанные данные
    sqlx::query("DELETE FROM user_roles WHERE user_id = $1")
        .bind(user_id)
        .execute(pool.get_ref())
        .await
        .ok();

    // Удаляем пользователя
    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool.get_ref())
        .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(ApiResponse::<()> {
                    success: true,
                    data: None,
                    message: Some("Пользователь успешно удален".to_string()),
                })
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()> {
                    success: false,
                    data: None,
                    message: Some("Пользователь не найден".to_string()),
                })
            }
        }
        Err(e) => {
            eprintln!("❌ Ошибка при удалении пользователя: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при удалении пользователя".to_string()),
            })
        }
    }
}

// Получение запросов в поддержку
pub async fn get_support_requests(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query(
        "SELECT sr.*, u.username as user_name 
         FROM support_requests sr
         JOIN users u ON sr.user_id = u.id
         ORDER BY sr.created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            let requests: Vec<serde_json::Value> = rows.iter().map(|row| {
                serde_json::json!({
                    "id": row.get::<Uuid, &str>("id"),
                    "user_id": row.get::<Uuid, &str>("user_id"),
                    "user_name": row.get::<String, &str>("user_name"),
                    "subject": row.get::<String, &str>("subject"),
                    "message": row.get::<String, &str>("message"),
                    "status": row.get::<String, &str>("status"),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, &str>("created_at"),
                    "updated_at": row.get::<chrono::DateTime<chrono::Utc>, &str>("updated_at")
                })
            }).collect();

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(requests),
                message: None,
            })
        }
        Err(e) => {
            eprintln!("❌ Ошибка при получении запросов поддержки: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Ошибка при получении запросов".to_string()),
            })
        }
    }
}

// Получение последней активности
pub async fn get_recent_activity(pool: web::Data<PgPool>) -> HttpResponse {
    // Собираем активность из разных источников
    let mut activities = Vec::new();

    // Новые пользователи (последние 5)
    if let Ok(rows) = sqlx::query(
        "SELECT username, created_at FROM users ORDER BY created_at DESC LIMIT 5"
    )
    .fetch_all(pool.get_ref())
    .await {
        for row in rows {
            activities.push(serde_json::json!({
                "id": Uuid::new_v4(),
                "icon": "👤",
                "text": format!("Новый пользователь: {}", row.get::<String, &str>("username")),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, &str>("created_at")
            }));
        }
    }

    // Последние новости
    if let Ok(rows) = sqlx::query(
        "SELECT title, created_at FROM news ORDER BY created_at DESC LIMIT 5"
    )
    .fetch_all(pool.get_ref())
    .await {
        for row in rows {
            activities.push(serde_json::json!({
                "id": Uuid::new_v4(),
                "icon": "📰",
                "text": format!("Опубликована новость: {}", row.get::<String, &str>("title")),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, &str>("created_at")
            }));
        }
    }

    // Сортируем по дате и берем 10 последних
    activities.sort_by(|a, b| {
        let a_date: chrono::DateTime<chrono::Utc> = serde_json::from_value(a["created_at"].clone()).unwrap();
        let b_date: chrono::DateTime<chrono::Utc> = serde_json::from_value(b["created_at"].clone()).unwrap();
        b_date.cmp(&a_date)
    });

    let recent_activities = activities.into_iter().take(10).collect::<Vec<_>>();

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(recent_activities),
        message: None,
    })
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>, // ДОБАВЬТЕ это поле
    pub role: Option<String>,
}