use crate::db::{
    schema::scores,
    users::{
        AssocUser,
        User,
    },
};
use chrono::NaiveDateTime;

mod repository;
pub use repository::Repository;
pub mod loader;

#[derive(Associations, Clone, Identifiable, Queryable)]
#[belongs_to(parent = "User")]
#[belongs_to(parent = "AssocUser", foreign_key = "user_id")]
pub struct Score {
    pub id: i32,
    pub user_id: String,
    pub clear_time: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "scores"]
pub struct ScoreNewForm {
    pub user_id: String,
    pub clear_time: i32,
}
