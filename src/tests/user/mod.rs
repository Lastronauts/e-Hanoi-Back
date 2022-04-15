use anyhow::Result;
use firestore_db_and_auth::{
    sessions::service_account::Session,
    users,
    Credentials,
    FirebaseAuthBearer,
};
use std::env;

mod get_user;
pub use get_user::get_user;
mod list_user;
pub use list_user::list_user;
mod create_user_in_db;
pub use create_user_in_db::create_user_in_db;
mod delete_user_in_db;
pub use delete_user_in_db::delete_user_in_db;

pub fn get_test_token() -> Result<String> {
    let credentials = Credentials::new(
        env::var("CREDENTIALS").unwrap().as_str(),
        &[
            env::var("SECURE_TOKEN_JWK").unwrap().as_str(),
            env::var("SERVICE_ACCOUNT_JWK").unwrap().as_str(),
        ],
    )?;
    let service_session = Session::new(credentials.clone())?;

    let session = users::sign_in(&service_session, "cargo-test@cargo-test.com", "cargo-test")?;
    let token = session.access_token();
    println!("user cargo_test's token is refreshed: {}", token);

    let session = users::sign_in(&service_session, "playground@playground.com", "playground")?;
    let token = session.access_token();
    println!("user playground's token is refreshed: {}", token);

    Ok(token)
}
