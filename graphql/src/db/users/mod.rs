use crate::db::schema::users;

mod repository;
pub use repository::Repository;
pub mod loader;

#[derive(Clone, Identifiable, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct AssocUser {
    pub id: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm {
    pub id: String,
    pub name: String,
}
