use actix_web::web::Data;
use anyhow::{
    Context,
    Result,
};
use diesel::{
    r2d2::ConnectionManager,
    PgConnection,
};
use r2d2::Pool;
use std::env;

mod schema;
pub mod users;
use users::loader::{
    create_users_loader,
    UsersLoader,
};
pub mod scores;
use scores::loader::{
    create_user_scores_loader,
    UserScoresLoader,
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn new_pool() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .context("failed to build database pool.")
}

pub struct Loaders {
    pub users: UsersLoader,
    pub user_scores: UserScoresLoader,
}

impl Loaders {
    pub fn new(pool: &Data<PgPool>) -> Self {
        Self {
            users: create_users_loader(pool),
            user_scores: create_user_scores_loader(pool),
        }
    }
}
