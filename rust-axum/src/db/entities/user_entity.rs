use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_with::{TimestampMilliSeconds, serde_as};

diesel::table! {
    users_axum (id) {
        id -> Integer,
        email -> VarChar,
        fname -> VarChar,
        lname -> VarChar,
        passwd -> Nullable<VarChar>,
        created_at -> Timestamp,
    }
}

#[serde_as]
#[derive(serde::Serialize, HasQuery)]
#[diesel(table_name = users_axum)]
pub struct UserModel {
    id: i32,
    email: String,
    fname: String,
    lname: String,
    passwd: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    created_at: NaiveDateTime,
}
