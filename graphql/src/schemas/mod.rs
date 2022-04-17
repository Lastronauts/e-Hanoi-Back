use crate::{
    auth::AuthCredentials,
    db::{
        Loaders,
        PgPool,
    },
};
use actix_web::web::Data;
use juniper::EmptySubscription;

pub mod root;
use root::{
    Mutation,
    Query,
    Schema,
};
pub mod score;
pub mod user;

pub struct Context {
    pub token: Option<String>,
    pub loaders: Loaders,
    pub pool: Data<PgPool>,
    pub credentials: Data<AuthCredentials>,
}

impl juniper::Context for Context {}

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::<Context>::new())
}
