use crate::db::{
    schema::scores::dsl::*,
    scores::{
        Score,
        ScoreNewForm,
    },
    PgPool,
};
use actix_web::web::Data;
use anyhow::Result;
use diesel::{
    debug_query,
    dsl::{
        delete,
        insert_into,
    },
    pg::Pg,
    prelude::*,
};
use log::debug;

pub struct Repository;

impl Repository {
    pub fn all(pool: &Data<PgPool>) -> Result<Vec<Score>> {
        let connection = pool.get()?;
        let query = scores.order(clear_time.asc());

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn any(pool: &Data<PgPool>, keys: &[i32]) -> Result<Vec<Score>> {
        let connection = pool.get()?;
        let query = scores.filter(id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_by_id(pool: &Data<PgPool>, key_id: i32) -> Result<Score> {
        let connection = pool.get()?;
        let query = scores.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn find_by_user_id(pool: &Data<PgPool>, key_user_id: &str) -> Result<Vec<Score>> {
        let connection = pool.get()?;
        let query = scores.filter(user_id.eq(key_user_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_best_by_user_id(pool: &Data<PgPool>, key_user_id: &str) -> Result<Score> {
        let connection = pool.get()?;
        let query = scores.filter(user_id.eq(key_user_id)).order(clear_time.asc());

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn insert(pool: &Data<PgPool>, score_form: ScoreNewForm) -> Result<Score> {
        let connection = pool.get()?;
        let query = insert_into(scores).values(score_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn delete(pool: &Data<PgPool>, key_id: i32) -> Result<Score> {
        let connection = pool.get()?;
        let query = delete(scores.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn delete_by_user_id(pool: &Data<PgPool>, key_user_id: &str) -> Result<Vec<Score>> {
        let connection = pool.get()?;
        let query = delete(scores.filter(user_id.eq(key_user_id)));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }
}
