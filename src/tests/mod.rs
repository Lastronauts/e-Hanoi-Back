use crate::main;
use anyhow::Result;
use dotenv::dotenv;
use std::{
    env,
    thread,
};

mod user;
use user::get_test_token;
mod score;

#[actix_rt::test]
async fn main_test() -> Result<()> {
    print!("\n\n");

    dotenv().ok();

    let protocol = "http";
    let host = match env::var("HOST") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_HOST")?,
    };
    let port = match env::var("PORT") {
        Ok(ok) => ok,
        Err(_) => env::var("LOCAL_PORT")?,
    };
    let address = format!("{}:{}", host, port);
    let graphql_endpoint = format!("{}://{}/graphql", protocol, address);

    let _ = thread::spawn(|| {
        main().unwrap();
    });

    std::thread::sleep(std::time::Duration::from_secs(5));

    let test_token = get_test_token()?;

    user::create_user_in_db(&graphql_endpoint, &test_token).await?;
    user::get_user(&graphql_endpoint, &test_token).await?;
    user::list_user(&graphql_endpoint).await?;
    score::create_score(&graphql_endpoint, &test_token).await?;
    score::get_my_best_score(&graphql_endpoint, &test_token).await?;
    score::ranking_score(&graphql_endpoint).await?;
    score::delete_score(&graphql_endpoint, &test_token).await?;
    user::delete_user_in_db(&graphql_endpoint, &test_token).await?;

    print!("\n\n");

    Ok(())
}
