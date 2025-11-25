mod auth;
mod database;
mod handlers;
mod models;
mod middleware;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
// Убираем неиспользуемый импорт
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("🚀 Запуск сервера SibWinterCraft...");

    let pool = match database::create_pool().await {
        Ok(pool) => {
            println!("✅ База данных готова к работе");
            pool
        }
        Err(e) => {
            eprintln!("❌ Ошибка подключения к базе данных: {}", e);
            return Err(e.into());
        }
    };

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);
    // let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("🌐 Сервер запущен на http://{}", server_url);
    println!("📊 API доступно по адресу: http://{}/api", server_url);

    // Создаем middleware
    // let auth_middleware = HttpAuthentication::bearer(middleware::auth_validator);
    // let admin_middleware = HttpAuthentication::bearer(middleware::admin_validator);

    HttpServer::new(move || {
        let cors = Cors::permissive(); 

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            // Публичные маршруты
            .service(
                web::scope("/api")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login))
                    .route("/games", web::get().to(handlers::get_games))
                    .route("/games/{id}", web::get().to(handlers::get_game))
                    .route("/news", web::get().to(handlers::get_news))
                    .route("/news", web::post().to(handlers::create_news))
                    .route("/support/{user_id}", web::post().to(handlers::create_support_request))
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/auth/me", web::get().to(handlers::get_current_user))
            )
            // Защищенные маршруты (требуют любой валидный JWT)
            // .service(
            //     web::scope("/api/protected")
            //         .wrap(auth_middleware.clone())
            //         // Здесь можно добавить защищенные маршруты для обычных пользователей
            // )
            // // Защищенные маршруты администратора (требуют роль admin)
            .service(
                web::scope("/api/admin")
                    //.wrap(admin_middleware.clone())
                    .route("/stats", web::get().to(handlers::get_admin_stats))
                    .route("/users", web::get().to(handlers::get_users))
                    .route("/users", web::post().to(handlers::create_user))
                    .route("/users/{id}", web::put().to(handlers::update_user))
                    .route("/users/{id}", web::delete().to(handlers::delete_user))
                    .route("/news", web::get().to(handlers::get_admin_news))
                    .route("/news/{id}", web::put().to(handlers::update_news))
                    .route("/news/{id}", web::delete().to(handlers::delete_news))
                    .route("/games", web::post().to(handlers::create_game))
                    .route("/games/{id}", web::put().to(handlers::update_game))
                    .route("/games/{id}", web::delete().to(handlers::delete_game))
                    .route("/support-requests", web::get().to(handlers::get_support_requests))
                    .route("/activity", web::get().to(handlers::get_recent_activity))
            )
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}