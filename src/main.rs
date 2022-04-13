use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{
        Compress,
        Logger,
    },
    web::{
        self,
        Data,
    },
    App,
    HttpServer,
};
use anyhow::Result;
use dotenv::dotenv;
use graphql::{
    auth::new_credentials,
    db::new_pool,
    graphiql,
    graphql,
    playground,
    schemas::create_schema,
};
use std::{
    env,
    sync::Arc,
};

#[cfg(test)]
mod tests;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let schema = Arc::new(create_schema());
    let pool = Arc::new(new_pool()?);
    let credentials = Arc::new(new_credentials()?);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(pool.clone()))
            .app_data(Data::from(credentials.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    });

    let host = match env::var("HOST") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_HOST")?,
    };
    let port = match env::var("PORT") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_PORT")?,
    };
    let address = format!("{}:{}", host, port);
    server = server.bind(address)?;
    server.run().await?;

    Ok(())
}
