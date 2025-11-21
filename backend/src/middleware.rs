// middleware.rs
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use crate::models::Claims;

// Middleware для проверки JWT
pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    println!("🔐 Auth middleware: получен токен длиной {}", token.len());
    
    match validate_jwt(token) {
        Ok(claims) => {
            println!("✅ Токен валиден: пользователь {}, роль {}", claims.username, claims.role);
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => {
            println!("❌ Ошибка валидации токена: {}", e);
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

// Middleware для проверки роли администратора
pub async fn admin_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    println!("🔐 Admin middleware: проверка прав администратора");
    
    match validate_jwt(token) {
        Ok(claims) => {
            println!("👤 Пользователь {} имеет роль: {}", claims.username, claims.role);
            if claims.role == "admin" {
                println!("✅ Доступ разрешен для администратора");
                req.extensions_mut().insert(claims);
                Ok(req)
            } else {
                println!("❌ Доступ запрещен: требуется роль admin");
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(e) => {
            println!("❌ Ошибка валидации токена в admin middleware: {}", e);
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "fallback-secret-key".to_string());
    decode::<Claims>(
        token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    ).map(|data| data.claims)
}