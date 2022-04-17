use crate::db::{
    scores::{
        Repository,
        Score,
    },
    users::AssocUser,
    PgPool,
};
use actix_web::web::Data;
use async_trait::async_trait;
use dataloader::{
    cached::Loader,
    BatchFn,
};
use diesel::prelude::GroupedBy;
use log::error;
use std::collections::HashMap;

fn create_assoc_users(keys: Vec<String>) -> Vec<AssocUser> {
    keys.into_iter().map(|k| AssocUser { id: k }).collect()
}

pub struct LoadFn {
    pub pool: Data<PgPool>,
}

impl LoadFn {
    pub fn user_scores(&self, keys: &[String]) -> Vec<Score> {
        Repository::any_user_scores(&self.pool, keys).unwrap_or_else(|e| {
            error!("{}", e);
            Vec::new()
        })
    }
}

#[async_trait]
impl BatchFn<String, Vec<Score>> for LoadFn {
    async fn load(&mut self, keys: &[String]) -> HashMap<String, Vec<Score>> {
        let assoc_users = create_assoc_users(keys.to_vec());
        let user_scores = self.user_scores(keys).grouped_by(&assoc_users[..]);

        assoc_users
            .iter()
            .zip(user_scores)
            .map(|p| (p.0.id.clone(), p.1))
            .collect()
    }
}

pub type UserScoresLoader = Loader<String, Vec<Score>, LoadFn>;

pub fn create_user_scores_loader(pool: &Data<PgPool>) -> UserScoresLoader {
    Loader::new(LoadFn { pool: pool.clone() }).with_yield_count(100)
}
