use crate::db::{
    schema::users::dsl::*,
    users::{
        User,
        UserNewForm,
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
    pub fn all(pool: &Data<PgPool>) -> Result<Vec<User>> {
        let connection = pool.get()?;

        Ok(users.load(&connection)?)
    }

    pub fn any(pool: &Data<PgPool>, keys: &[String]) -> Result<Vec<User>> {
        let connection = pool.get()?;
        let query = users.filter(id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_by_id(pool: &Data<PgPool>, key_id: &str) -> Result<User> {
        let connection = pool.get()?;
        let query = users.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn find_by_name(pool: &Data<PgPool>, key_name: &str) -> Result<Vec<User>> {
        let connection = pool.get()?;
        let query = users.filter(name.eq(key_name));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn insert(pool: &Data<PgPool>, user_form: UserNewForm) -> Result<User> {
        let connection = pool.get()?;
        let query = insert_into(users).values(user_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn delete(pool: &Data<PgPool>, key_id: &str) -> Result<User> {
        let connection = pool.get()?;
        let query = delete(users.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }
}
