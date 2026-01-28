use crate::{
    app::app_response::AppResponse,
    db::{
        db_handle_error,
        entities::user_entity::{UserEntity, users_axum},
        models::user_model::UserModel,
    },
};

use crate::db::entities::user_entity::users_axum::dsl;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct UserService {}

#[allow(unused)]
impl UserService {
    /// Search user by fname and lname
    pub async fn search(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        fname: Option<String>,
        lname: Option<String>,
    ) -> Result<Vec<UserModel>, AppResponse> {
        if fname.is_none() && lname.is_none() {
            return Ok(vec![]);
        }
        let result: Vec<UserModel> = UserEntity::query()
            .filter(dsl::fname.ilike(format!("%{}%", fname.unwrap_or("".to_string()))))
            .filter(dsl::lname.ilike(format!("%{}%", lname.unwrap_or("".to_string()))))
            .load(&mut conn)
            .await
            .map_err(db_handle_error)?
            .iter()
            .map(|e| UserModel::from_entity(e))
            .collect();
        Ok(result)
    }

    /// Find entity by id
    pub async fn find_by_id(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<UserModel, AppResponse> {
        let result = UserEntity::query()
            .find(id)
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }

    /// Find entity by email
    pub async fn find_by_email(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        email: String,
    ) -> Result<UserModel, AppResponse> {
        let result: UserEntity = UserEntity::query()
            .filter(dsl::email.eq(email))
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }

    /// Create new entity
    pub async fn insert(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        user: &UserModel,
    ) -> Result<UserModel, AppResponse> {
        let entity = user.as_new_entity();
        let result = diesel::insert_into(users_axum::table)
            .values(entity)
            .returning(UserEntity::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }
}
