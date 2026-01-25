use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;

use crate::{
    app::app_error::AppError,
    db::{self, entities::user_entity::{UserModel, UserNewModel, users_axum}},
};

pub struct UserService {}

impl UserService {
    pub async fn new(user: UserNewModel, pool: &Pool<Manager<PgConnection>>) -> Result<UserModel, AppError> {
        // Get manager
        let poll = db::get_poll(pool).await?;
        // Query
        let result = poll.interact(|conn| {
            diesel::insert_into(users_axum::table)
                .values(user)
                .returning(UserModel::as_returning())
                .get_result(conn)
        }).await;
        // Result
        Ok(db::map_poll(result)?)
    }

    pub async fn get_list(pool: &Pool<Manager<PgConnection>>) -> Result<Vec<UserModel>, AppError> {
        // Get manager
        let poll = db::get_poll(pool).await?;
        // Query
        let result = poll.interact(|conn| UserModel::query().load(conn)).await;
        // Result
        Ok(db::map_poll(result)?)
    }
}
