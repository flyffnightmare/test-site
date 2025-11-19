mod auth;
mod database;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Убрали env_logger::init() - он не нужен для базовой работы

    println!("🚀 Запуск сервера Game Company...");

    // Создаем пул соединений (миграции выполняются внутри)
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

    println!("🌐 Сервер запущен на http://{}", server_url);
    println!("📊 API доступно по адресу: http://{}/api", server_url);
    println!("🎮 Эндпоинты:");
    println!("   POST /api/register - регистрация");
    println!("   POST /api/login - вход");
    println!("   GET  /api/games - список игр");
    println!("   GET  /api/health - проверка здоровья");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            // Убрали Logger так как он требует env_logger
            .route("/api/register", web::post().to(handlers::register))
            .route("/api/login", web::post().to(handlers::login))
            .route("/api/games", web::get().to(handlers::get_games))
            .route(
                "/api/health",
                web::get().to(|| async { "✅ Сервер работает нормально" }),
            )
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}
