use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;

use crate::{
    app::app_error::AppError,
    db::{
        self,
        entities::user_entity::{UserEntity, users_axum},
        models::user_model::UserModel,
    },
};

pub struct UserService {}

impl UserService {
    pub async fn new(
        user: &UserModel,
        pool: &Pool<Manager<PgConnection>>,
    ) -> Result<UserModel, AppError> {
        // Get manager
        let poll = db::get_poll(pool).await?;
        // Get entity for insert
        let entity = user.as_new_entity();
        // Query
        let result = poll
            .interact(|conn| {
                diesel::insert_into(users_axum::table)
                    .values(entity)
                    .returning(UserEntity::as_returning())
                    .get_result(conn)
            })
            .await;
        // Result
        Ok(UserModel::from_entity(&db::map_poll(result)?))
    }

    pub async fn get_list(pool: &Pool<Manager<PgConnection>>) -> Result<Vec<UserModel>, AppError> {
        // Get manager
        let poll = db::get_poll(pool).await?;
        // Query
        let result = poll.interact(|conn| UserEntity::query().load(conn)).await;
        // Result
        Ok(db::map_poll(result)?
            .iter()
            .map(|e| UserModel::from_entity(e))
            .collect())
    }
}
