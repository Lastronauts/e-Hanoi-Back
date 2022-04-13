use crate::db::schema::users;

pub mod loader;
mod repository;
pub use repository::Repository;

#[derive(Clone, Identifiable, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm {
    pub id: String,
    pub name: String,
}
