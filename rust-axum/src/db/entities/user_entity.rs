use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use passwords::{PasswordGenerator};
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::utils;

diesel::table! {
    users_axum (id) {
        id -> Integer,
        email -> VarChar,
        fname -> VarChar,
        lname -> VarChar,
        password -> Nullable<VarChar>,
        created_at -> Timestamp,
    }
}

#[serde_as]
#[derive(Serialize, HasQuery)]
#[diesel(table_name = users_axum)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub password: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

#[allow(dead_code)]
impl UserModel {
    /// Validate user password
    pub fn password_validate(&self, password: &str) -> bool {
        if let Some(hash) = &self.password {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&hash).expect("error parse hash");
            return match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                Ok(()) => true,
                Err(_) => false,
            };
        }
        false
    }
}

#[serde_as]
#[derive(Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = users_axum)]
pub struct UserNewModel {
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub password: Option<String>,
    #[serde(default = "utils::time::now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

impl UserNewModel {
    /// Create password hash
    pub fn password_update(&mut self, salt: &str) -> String {
        let pg = PasswordGenerator::new()
            .length(12)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .strict(true);
        let password = pg.generate_one().unwrap();
        let salt = SaltString::from_b64(&salt).expect("error create salt");
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt).expect("error create hash").to_string();
        self.password = Some(hash);
        password
    }
}
