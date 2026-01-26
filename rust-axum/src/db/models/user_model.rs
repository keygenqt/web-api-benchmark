use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::SaltString,
    password_hash::rand_core::OsRng,
};
use chrono::NaiveDateTime;
use passwords::PasswordGenerator;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::user_entity::{UserEntity, UserNewEntity},
    utils::{self, constants},
};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
    pub id: Option<i32>,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub pwd_hash: Option<String>,
    pub password: Option<String>,
    #[serde(default = "utils::time::now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

#[allow(dead_code)]
impl UserModel {
    pub fn from_entity(entity: &UserEntity) -> Self {
        Self {
            id: Some(entity.id),
            email: entity.email.clone(),
            fname: entity.fname.clone(),
            lname: entity.lname.clone(),
            pwd_hash: entity.pwd_hash.clone(),
            password: None,
            created_at: entity.created_at,
        }
    }

    pub fn from_new_entity(entity: &UserNewEntity) -> Self {
        Self {
            id: None,
            email: entity.email.clone(),
            fname: entity.fname.clone(),
            lname: entity.lname.clone(),
            pwd_hash: entity.pwd_hash.clone(),
            password: None,
            created_at: entity.created_at,
        }
    }

    pub fn as_entity(&self) -> UserEntity {
        UserEntity {
            id: self.id.expect("is new entity"),
            email: self.email.clone(),
            fname: self.fname.clone(),
            lname: self.lname.clone(),
            pwd_hash: self.pwd_hash.clone(),
            created_at: self.created_at,
        }
    }

    pub fn as_new_entity(&self) -> UserNewEntity {
        UserNewEntity {
            email: self.email.clone(),
            fname: self.fname.clone(),
            lname: self.lname.clone(),
            pwd_hash: self.pwd_hash.clone(),
            created_at: self.created_at,
        }
    }

    pub fn is_new(&self) -> bool {
        self.id.is_none()
    }

    /// Create password hash
    pub fn password_up(&mut self) {
        let pg = PasswordGenerator::new()
            .length(12)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .strict(true);
        let password = pg.generate_one().unwrap();
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
        self.password = Some(password);
    }

    /// Validate user password
    pub fn password_valid(&self, password: &str) -> bool {
        if let Some(hash) = &self.password {
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
