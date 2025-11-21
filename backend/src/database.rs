use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::env;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

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

    // Создаем таблицу users ПЕРВОЙ
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

    // Создаем таблицу games
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
            platform VARCHAR(100),
            steam_url VARCHAR(500),
            release_date VARCHAR(50),
            developer VARCHAR(255),
            publisher VARCHAR(255),
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'games' создана/проверена");

    // Таблица новостей
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS news (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(500) NOT NULL,
            content TEXT NOT NULL,
            image_url VARCHAR(500),
            author_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'news' создана/проверена");

    // Таблица ролей пользователей
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_roles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(user_id, role)
        )
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'user_roles' создана/проверена");

    // Таблица запросов в поддержку
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS support_requests (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            subject VARCHAR(500) NOT NULL,
            message TEXT NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'open',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;
    println!("✅ Таблица 'support_requests' создана/проверена");

    // Создаем админ-пользователя если его нет
    create_admin_user(pool).await?;

    // Добавляем демо-игру
    create_demo_game(pool).await?;

    // Создаем демо-новости
    create_demo_news(pool).await?;

    println!("🎉 Все миграции успешно выполнены!");
    Ok(())
}

async fn create_admin_user(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("👤 Проверка админ-пользователя...");

    // Используем обычный query вместо query!
    let admin_exists_result = sqlx::query(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = 'admin') as exists"
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = admin_exists_result {
        let exists: bool = row.get("exists");
        
        if !exists {
            println!("📝 Создание админ-пользователя...");
            
            let password_hash = hash("admin123", DEFAULT_COST)
                .expect("Failed to hash admin password");

            let admin_id = Uuid::new_v4();

            sqlx::query(
                "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)"
            )
            .bind(admin_id)
            .bind("admin")
            .bind("admin@sibwintercraft.com")
            .bind(&password_hash)
            .execute(pool)
            .await?;

            // Добавляем роль админа
            sqlx::query(
                "INSERT INTO user_roles (user_id, role) VALUES ($1, $2)"
            )
            .bind(admin_id)
            .bind("admin")
            .execute(pool)
            .await?;

            println!("✅ Админ-пользователь создан");
            println!("   👤 Логин: admin");
            println!("   🔑 Пароль: admin123");
            println!("   📧 Email: admin@sibwintercraft.com");
        } else {
            println!("✅ Админ-пользователь уже существует");
        }
    } else {
        println!("❌ Не удалось проверить существование админ-пользователя");
    }

    Ok(())
}

async fn create_demo_game(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("🎮 Проверка демо-игры...");

    let game_count_result = sqlx::query("SELECT COUNT(*) as count FROM games")
        .fetch_one(pool)
        .await?;

    let count: i64 = game_count_result.get("count");

    if count == 0 {
        println!("📝 Добавление демо-игры...");
        sqlx::query(
            r#"
            INSERT INTO games (
                title, 
                short_description, 
                full_description, 
                image_url, 
                screenshots,
                genre,
                platform, 
                steam_url,
                release_date,
                developer,
                publisher
            ) VALUES (
                'Tales of Wizeria',
                'Увлекательный платформер в волшебном мире магии и приключений',
                'Tales of Wizeria - это захватывающий платформер, который перенесет вас в зачарованный мир, полный тайн и опасностей. Играйте за юного волшебника, которому предстоит пройти через разнообразные уровни, решать головоломки и сражаться с магическими существами.

Основные особенности:
• Динамичный геймплей платформера с элементами головоломок
• Разнообразные магические способности и улучшения
• Уникальные враги и боссы на каждом уровне
• Захватывающая история о спасении магического королевства
• Великолепная пиксельная графика с современными эффектами
• Система коллекционных предметов и достижений

Отправляйтесь в незабываемое путешествие по миру Wizeria, где каждая платформа скрывает новые секреты, а каждое препятствие проверяет ваши навыки!',
                '/images/games/tales-of-wizeria/main.jpg',
                ARRAY[
                    '/images/games/tales-of-wizeria/screenshot1.jpg',
                    '/images/games/tales-of-wizeria/screenshot2.jpg', 
                    '/images/games/tales-of-wizeria/screenshot3.jpg',
                    '/images/games/tales-of-wizeria/screenshot4.jpg'
                ],
                'Платформер',
                'Windows, Linux',
                'https://store.steampowered.com/app/1234567/Tales_of_Wizeria/',
                '2027',
                'SibWinterCraft',
                'SibWinterCraft'
            )
            "#
        ).execute(pool).await?;
        println!("✅ Демо-игра добавлена");
    } else {
        println!("✅ Игры уже существуют в базе");
    }

    Ok(())
}

async fn create_demo_news(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("📰 Проверка демо-новостей...");

    let news_count_result = sqlx::query("SELECT COUNT(*) as count FROM news")
        .fetch_one(pool)
        .await?;

    let count: i64 = news_count_result.get("count");

    if count == 0 {
        println!("📝 Добавление демо-новостей...");
        
        // Получаем ID админ-пользователя с обычным query
        let admin_user_result = sqlx::query("SELECT id FROM users WHERE username = 'admin' LIMIT 1")
            .fetch_optional(pool)
            .await?;

        if let Some(row) = admin_user_result {
            let admin_id: Uuid = row.get("id");
            
            sqlx::query(
                r#"
                INSERT INTO news (title, content, image_url, author_id) VALUES
                ('Tales of Wizeria выходит в ранний доступ!', 'Мы рады сообщить, что Tales of Wizeria теперь доступна в раннем доступе на Steam! Присоединяйтесь к приключению и помогите нам сделать игру еще лучше своими отзывами. В раннем доступе вас ждут первые 3 главы игры, 15 уникальных уровней и 5 боссов. Мы будем регулярно обновлять игру на основе ваших отзывов!', '/images/news/tow-early-access.jpg', $1),
                ('Новые локации в разработке', 'Команда разработчиков активно работает над добавлением новых захватывающих локаций в Tales of Wizeria. Скоро вы сможете исследовать Зачарованные леса и Ледяные пещеры! Каждая новая локация будет содержать уникальные механики, врагов и секреты. Следите за обновлениями!', '/images/news/new-locations.jpg', $1),
                ('Добро пожаловать на наш новый сайт!', 'Мы запустили совершенно новый сайт SibWinterCraft! Теперь вы можете следить за нашими проектами, читать новости и быть в курсе всех событий. На сайте вы найдете информацию о наших играх, блог разработчиков и возможность связаться с поддержкой. Оставайтесь на связи!', '/images/news/new-website.jpg', $1)
                "#
            )
            .bind(admin_id)
            .execute(pool)
            .await?;
            println!("✅ Демо-новости добавлены");
        } else {
            println!("❌ Админ-пользователь не найден для создания новостей");
        }
    } else {
        println!("✅ Новости уже существуют в базе");
    }

    Ok(())
}