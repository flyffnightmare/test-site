# Game Company Website

Веб-сайт для игровой компании с фронтендом на Vue.js и бэкендом на Rust.

## Структура проекта

game-company/
├── frontend/ # Vue.js приложение
├── backend/ # Rust сервер
├── docker-compose.yml # Конфигурация PostgreSQL
└── README.md


## Технологии

- **Фронтенд**: Vue.js 3, Vue Router
- **Бэкенд**: Rust, Actix-web, SQLx
- **База данных**: PostgreSQL
- **Аутентификация**: JWT (базовая реализация)

## Установка и запуск

### Предварительные требования

- Rust и Cargo
- Node.js и npm
- Docker и Docker Compose

### Запуск проекта

1. **Клонируйте репозиторий:**
```bash
git clone <url-репозитория>
cd game-company

2. Запустите PostgreSQL:

docker-compose up -d

3. Настройте бэкенд:

cd backend
cp .env.example .env
# Отредактируйте .env при необходимости
cargo run

4. Настройте фронтенд:

cd frontend
npm install
npm run dev

5. Откройте в браузере:
http://localhost:5173

API Endpoints
POST /api/register - Регистрация пользователя

POST /api/login - Вход пользователя

GET /api/games - Получить список игр

GET /api/health - Проверка здоровья сервера

# 3. Создаем примеры файлов окружения

**backend/.env.example**
```env
DATABASE_URL=postgres://user:password@localhost:5432/game_company
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
SERVER_HOST=127.0.0.1
SERVER_PORT=8080