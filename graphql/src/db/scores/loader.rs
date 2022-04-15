use crate::db::{
    scores::{
        Repository,
        Score,
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
    pub fn scores(&self, keys: &[i32]) -> Vec<Score> {
        Repository::any(&self.pool, keys).unwrap_or_else(|e| {
            error!("{}", e);
            Vec::new()
        })
    }
}

#[async_trait]
impl BatchFn<i32, Score> for LoadFn {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Score> {
        self.scores(keys).into_iter().map(|p| (p.id, p)).collect()
    }
}

pub type ScoresLoader = Loader<i32, Score, LoadFn>;

pub fn create_scores_loader(pool: &Data<PgPool>) -> ScoresLoader {
    Loader::new(LoadFn { pool: pool.clone() }).with_yield_count(100)
}
