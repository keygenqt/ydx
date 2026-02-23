use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString,
    password_hash::rand_core::OsRng,
};
use chrono::NaiveDateTime;

use crate::{
    db::entities::user_entity::{NewUserEntity, UserEntity},
    utils,
};

/// Model entry [UserEntity]
#[derive(Clone, Debug)]
pub struct UserModel {
    pub id: Option<i32>,
    pub email: String,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub hash_password: String,
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl UserModel {
    pub fn new(email: &String, password: &String) -> Self {
        Self {
            id: None,
            email: email.clone(),
            icon: None,
            fname: None,
            lname: None,
            hash_password: Self::get_hash_password(&password),
            created_at: utils::local_time::chrono_time_now(),
        }
    }

    /// Create model from [UserEntity]
    pub fn from_entity(entity: &UserEntity) -> Self {
        Self {
            id: Some(entity.id),
            email: entity.email.clone(),
            icon: entity.icon.clone(),
            fname: entity.fname.clone(),
            lname: entity.lname.clone(),
            hash_password: entity.hash_password.clone(),
            created_at: entity.created_at,
        }
    }

    /// Create new [NewUserEntity]
    pub fn to_new_entity(&self) -> NewUserEntity {
        NewUserEntity {
            email: self.email.clone(),
            icon: self.icon.clone(),
            fname: self.fname.clone(),
            lname: self.lname.clone(),
            hash_password: self.hash_password.clone(),
            created_at: self.created_at.clone(),
        }
    }

    /// Validate password
    pub fn password_valid(&self, password: &str) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.hash_password).expect("error parse hash");
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Check is empty model
    pub fn is_empty(&self) -> bool {
        self.email.is_empty()
    }

    /// Create hash from password for db
    fn get_hash_password(password: &String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("error create password")
            .to_string()
    }
}
