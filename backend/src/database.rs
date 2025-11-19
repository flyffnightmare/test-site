use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("🔗 Подключение к базе данных...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Подключение к базе данных установлено");

    // Запускаем миграции
    run_migrations(&pool).await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("🔄 Выполнение миграций...");

    // Создаем таблицу users
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("✅ Таблица 'users' создана/проверена");

    // Создаем таблицу games с REAL вместо NUMERIC/DECIMAL
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS games (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(255) NOT NULL,
            description TEXT,
            image_url VARCHAR(500),
            genre VARCHAR(100),
            price REAL DEFAULT 0.00,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("✅ Таблица 'games' создана/проверена");

    // Проверяем, есть ли уже игры в таблице
    let result = sqlx::query("SELECT COUNT(*) as count FROM games")
        .fetch_one(pool)
        .await?;
    let count: i64 = result.get("count");

    if count == 0 {
        println!("📝 Добавление демо-игр...");
        sqlx::query(
            r#"
            INSERT INTO games (id, title, description, image_url, genre, price) VALUES
            (gen_random_uuid(), 'Cyber Adventure', 'Захватывающее киберпанк приключение', '/images/cyber-adventure.jpg', 'RPG', 29.99),
            (gen_random_uuid(), 'Space Warriors', 'Эпическая битва в космосе', '/images/space-warriors.jpg', 'Strategy', 19.99),
            (gen_random_uuid(), 'Fantasy Quest', 'Фэнтезийный квест с магией', '/images/fantasy-quest.jpg', 'Adventure', 24.99)
            "#
        ).execute(pool).await?;
        println!("✅ Демо-игры добавлены");
    }

    println!("🎉 Все миграции успешно выполнены!");
    Ok(())
}
