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

impl Default for UserModel {
    fn default() -> Self {
        Self {
            id: None,
            email: "".to_string(),
            icon: None,
            fname: None,
            lname: None,
            hash_password: "".to_string(),
            created_at: utils::local_time::chrono_time_now(),
        }
    }
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
        Self::default().with_entity(entity)
    }

    /// Update model from [UserEntity]
    pub fn update_from_entity(&mut self, entity: &UserEntity) {
        self.id = Some(entity.id);
        self.email = entity.email.clone();
        self.icon = entity.icon.clone();
        self.fname = entity.fname.clone();
        self.lname = entity.lname.clone();
        self.hash_password = entity.hash_password.clone();
        self.created_at = entity.created_at;
    }

    /// Update model from [UserEntity]
    pub fn with_entity(mut self, entity: &UserEntity) -> Self {
        self.update_from_entity(entity);
        self
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
