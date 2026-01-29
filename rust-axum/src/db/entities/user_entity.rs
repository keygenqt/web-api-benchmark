use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

// Table users
diesel::table! {
    users_axum (id) {
        id -> Integer,
        email -> VarChar,
        fname -> VarChar,
        lname -> VarChar,
        pwd_hash -> Nullable<VarChar>,
        create_at -> Timestamp,
    }
}

/// Entity
#[serde_as]
#[derive(Serialize, HasQuery)]
#[diesel(table_name = users_axum)]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub pwd_hash: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub create_at: NaiveDateTime,
}

/// Entity new
#[serde_as]
#[derive(Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = users_axum)]
pub struct UserNewEntity {
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub pwd_hash: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub create_at: NaiveDateTime,
}
