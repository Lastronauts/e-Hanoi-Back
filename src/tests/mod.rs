use crate::*;
use anyhow::Result;
use dotenv::dotenv;
use std::thread;

mod user;

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

    user::get_user(&graphql_endpoint).await?;
    user::list_user(&graphql_endpoint).await?;
    user::create_user_on_db(&graphql_endpoint).await?;
    user::delete_user_on_db(&graphql_endpoint).await?;

    print!("\n\n");

    Ok(())
}
