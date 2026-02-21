use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString,
    password_hash::rand_core::OsRng,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::user_entity::{NewUserEntity, UserEntity},
    utils,
};

/// Model entry [UserEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
    pub id: Option<i32>,
    pub email: String,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
    // user pass
    #[serde(skip_serializing)]
    pub password: Option<String>,
    // for database
    #[serde(skip_serializing)]
    pub pwd_hash: Option<String>,
}

/// Impl model entry
impl UserModel {
    pub fn new(email: &String, password: &String) -> Self {
        let mut model = Self {
            id: None,
            email: email.clone(),
            icon: None,
            fname: None,
            lname: None,
            pwd_hash: None,
            password: Some(password.clone()),
            created_at: utils::local_time::chrono_time_now(),
        };
        model.update_hash();
        model
    }

    /// Create hash from password for db
    pub fn update_hash(&mut self) {
        let password = self.password.clone().expect("password expected");
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("error create hash")
            .to_string();
        self.pwd_hash = Some(hash);
    }

    /// Validate password for auth
    pub fn password_valid(&self, password: &str) -> bool {
        if let Some(hash) = &self.pwd_hash {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&hash).expect("error parse hash");
            return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                Ok(()) => true,
                Err(_) => false,
            };
        }
        false
    }

    /// Convert from database entity to model
    pub fn from_entity(entity: &UserEntity) -> Self {
        Self {
            id: Some(entity.id),
            email: entity.email.clone(),
            icon: entity.icon.clone(),
            fname: entity.fname.clone(),
            lname: entity.lname.clone(),
            pwd_hash: Some(entity.pwd_hash.clone()),
            password: None,
            created_at: entity.created_at,
        }
    }

    /// Convert to new entity for database insertion
    pub fn to_new_entity(&self) -> NewUserEntity {
        NewUserEntity {
            email: self.email.clone(),
            icon: self.icon.clone(),
            fname: self.fname.clone(),
            lname: self.lname.clone(),
            pwd_hash: self
                .pwd_hash
                .clone()
                .expect("password hash is required for creating new user"),
            created_at: self.created_at,
        }
    }
}
