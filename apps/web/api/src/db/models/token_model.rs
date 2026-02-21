use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::{RngExt, distr::Alphanumeric, rng};

use crate::{
    app::app_user::AppUser, db::entities::token_entity::{NewTokenEntity, TokenEntity}, utils::{self}
};

/// Model entry [TokenEntity]
#[derive(Debug, Clone)]
pub struct TokenModel {
    pub id: Option<i32>,
    pub user_id: i32,
    pub device_uuid: String,
    pub hash_refresh_token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    // new tokens for response
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[allow(dead_code)]
/// Impl model entry
impl TokenModel {
    /// Create new [TokenModel]
    pub fn new(user_id: i32, uuid: String, secret: String) -> Self {
        let access_token = Self::get_access_token(user_id, &uuid, secret);
        let refresh_token: String = rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let hash_refresh_token = Self::get_hash_refresh_token(&refresh_token);
        Self {
            id: None,
            user_id,
            device_uuid: uuid.clone(),
            hash_refresh_token,
            expires_at: utils::local_time::chrono_time_now() + Duration::days(30),
            created_at: utils::local_time::chrono_time_now(),
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
        }
    }

    /// Update date from [TokenEntity]
    pub fn update_from_entity(&mut self, entity: &TokenEntity) {
        self.id = Some(entity.id);
        self.user_id = entity.user_id;
        self.device_uuid = entity.device_uuid.clone();
        self.hash_refresh_token = entity.hash_refresh_token.clone();
        self.expires_at = entity.expires_at;
        self.created_at = entity.created_at;
    }

    /// Update date from [TokenEntity]
    pub fn with_entity(mut self, entity: &TokenEntity) -> Self {
        self.update_from_entity(entity);
        self
    }

    /// Create new [NewTokenEntity]
    pub fn to_new_entity(&self) -> NewTokenEntity {
        NewTokenEntity {
            user_id: self.user_id,
            hash_refresh_token: self.hash_refresh_token.clone(),
            device_uuid: self.device_uuid.clone(),
            expires_at: self.expires_at,
        }
    }

    /// Validate refresh_token
    pub fn refresh_token_valid(&self, refresh_token: &str) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.hash_refresh_token).expect("error parse hash");
        return match argon2.verify_password(refresh_token.as_bytes(), &parsed_hash) {
            Ok(()) => true,
            Err(_) => false,
        };
    }

    /// Check expired
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now().naive_utc()
    }

    /// Create hash from refresh_token for db
    fn get_hash_refresh_token(refresh_token: &String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(refresh_token.as_bytes(), &salt)
            .expect("error create refresh_token")
            .to_string()
    }

    /// Generate JWT access token
    fn get_access_token(user_id: i32, device_uuid: &str, secret: String) -> String {
        let claims = AppUser {
            user_id,
            device_uuid: device_uuid.to_string(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("error create access_token")
    }
}
