use crate::db::{
    users::{
        Repository,
        User,
    },
    PgPool,
};
use actix_web::web::Data;
use async_trait::async_trait;
use dataloader::{
    cached::Loader,
    BatchFn,
};
use log::error;
use std::collections::HashMap;

pub struct LoadFn {
    pub pool: Data<PgPool>,
}

impl LoadFn {
    pub fn users(&self, keys: &[String]) -> Vec<User> {
        Repository::any(&self.pool, keys).unwrap_or_else(|e| {
            error!("{}", e);
            Vec::new()
        })
    }
}

#[async_trait]
impl BatchFn<String, User> for LoadFn {
    async fn load(&mut self, keys: &[String]) -> HashMap<String, User> {
        self.users(keys)
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect()
    }
}

pub type UsersLoader = Loader<String, User, LoadFn>;

pub fn create_users_loader(pool: &Data<PgPool>) -> UsersLoader {
    Loader::new(LoadFn { pool: pool.clone() }).with_yield_count(100)
}
