use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;

use crate::{
    app::app_error::AppError,
    db::{self, entities::user_entity::UserModel},
};

pub struct UserService {}

impl UserService {
    pub async fn get_list(pool: Pool<Manager<PgConnection>>) -> Result<Vec<UserModel>, AppError> {
        // Get manager
        let poll = db::get_poll(pool).await?;
        // Query
        let result = poll.interact(|conn| UserModel::query().load(conn)).await;
        // Result
        Ok(db::map_poll(result)?)
    }
}
