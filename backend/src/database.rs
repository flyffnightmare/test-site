use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

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
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'users' создана/проверена");

    // Создаем таблицу games с обновленной структурой
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS games (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(255) NOT NULL,
            short_description TEXT,
            full_description TEXT,
            image_url VARCHAR(500),
            screenshots TEXT[],
            genre VARCHAR(100),
            steam_url VARCHAR(500),
            release_date VARCHAR(50),
            developer VARCHAR(255),
            publisher VARCHAR(255),
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'games' создана/проверена");

    // Проверяем, есть ли уже игры в таблице
    let result = sqlx::query("SELECT COUNT(*) as count FROM games")
        .fetch_one(pool)
        .await?;
    let count: i64 = result.get("count");

    if count == 0 {
        println!("📝 Добавление игры Tails of Wizeria...");
        sqlx::query(
            r#"
            INSERT INTO games (
                title, 
                short_description, 
                full_description, 
                image_url, 
                screenshots,
                genre, 
                steam_url,
                release_date,
                developer,
                publisher
            ) VALUES (
                'Tails of Wizeria',
                'Захватывающее фэнтези-приключение в мире магии и древних тайн',
                'Tails of Wizeria - это эпическая RPG с открытым миром, где вы становитесь наследником древнего магического рода. Исследуйте огромный мир, полный загадок, сражайтесь с мифическими существами и раскройте секреты исчезнувшей цивилизации волшебников.

Особенности игры:
• Огромный открытый мир с разнообразными локациями
• Глубокая система прокачки персонажа и магии
• Динамичная боевая система с комбо-атаками
• Богатый сюжет с множеством побочных квестов
• Система крафта и улучшения снаряжения
• Возможность приручать магических существ

Погрузитесь в мир, где каждая тропинка ведет к новому приключению, а каждое решение влияет на судьбу всего королевства.',
                '/images/games/tails-of-wizeria/main.jpg',
                ARRAY[
                    '/images/games/tails-of-wizeria/screenshot1.jpg',
                    '/images/games/tails-of-wizeria/screenshot2.jpg', 
                    '/images/games/tails-of-wizeria/screenshot3.jpg',
                    '/images/games/tails-of-wizeria/screenshot4.jpg'
                ],
                'RPG',
                'https://store.steampowered.com/app/1234567/Tails_of_Wizeria/',
                '2024',
                'СибВинтерКрафт',
                'СибВинтерКрафт'
            )
            "#
        ).execute(pool).await?;
        println!("✅ Игра Tails of Wizeria добавлена");
    } else {
        println!("✅ Игры уже существуют в базе, пропускаем добавление");
    }

    println!("🎉 Все миграции успешно выполнены!");
    Ok(())
}