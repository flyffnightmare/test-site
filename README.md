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
```  
  
2. **Запустите PostgreSQL:**  
```bash
docker-compose up -d
```  

3. **Настройте бэкенд:**

```bash
cd backend
cp .env.example .env
# Отредактируйте .env при необходимости
cargo run
```  

4. **Настройте фронтенд:**  
```bash
cd frontend
npm install
npm run dev
```  

5. **Откройте в браузере:**
```bash
http://localhost:5173
```  

API Endpoints  
POST /api/register - Регистрация пользователя  
POST /api/login - Вход пользователя  
GET /api/games - Получить список игр  
GET /api/health - Проверка здоровья сервера  
