use actix_web::{
    http::header,
    web::{
        Data,
        Payload,
    },
    Error,
    HttpResponse,
};
use juniper_actix::{
    graphiql_handler,
    graphql_handler,
    playground_handler,
};

#[macro_use]
extern crate diesel;

pub mod auth;
use auth::AuthCredentials;
pub mod db;
use crate::db::{
    Loaders,
    PgPool,
};
pub mod resolvers;
pub mod schemas;
use crate::schemas::{
    root::Schema,
    Context,
};

pub async fn graphql(req: actix_web::HttpRequest, payload: Payload, schema: Data<Schema>, pool: Data<PgPool>, credentials: Data<AuthCredentials>) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .map(|t| t.to_str().unwrap().to_string());

    let context = Context {
        token,
        loaders: Loaders::new(&pool),
        pool,
        credentials,
    };

    graphql_handler(&schema, &context, req, payload).await
}

pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
