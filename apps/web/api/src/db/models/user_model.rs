use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString,
    password_hash::rand_core::OsRng,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::user_entity::{NewUserEntity, UserEntity},
    utils::{self, constants},
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
    #[serde(skip_serializing)]
    pub pwd_hash: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
impl UserModel {
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

    /// Create hash from password for db
    pub fn update_hash(&mut self) {
        let password = self.password.clone().expect("password expected");
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(
            constants::ARGON2_DEFAULT_M_COST, // default for spring Argon2
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            None,
        )
        .unwrap();
        let argon2 = Argon2::from(params);
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("error create hash")
            .to_string();
        self.pwd_hash = Some(hash);
    }

    /// Validate password for auth
    pub fn password_valid(&self, password: &str) -> bool {
        if let Some(hash) = &self.pwd_hash {
            let params = Params::new(
                constants::ARGON2_DEFAULT_M_COST,
                Params::DEFAULT_T_COST,
                Params::DEFAULT_P_COST,
                None,
            )
            .unwrap();
            let argon2 = Argon2::from(params);
            let parsed_hash = PasswordHash::new(&hash).expect("error parse hash");
            return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                Ok(()) => true,
                Err(_) => false,
            };
        }
        false
    }
}
