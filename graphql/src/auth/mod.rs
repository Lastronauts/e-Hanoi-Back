use anyhow::{
    bail,
    Result,
};
use firestore_db_and_auth::{
    sessions::user::Session,
    Credentials,
    UserSession,
};
use std::env;

pub type AuthCredentials = Credentials;

pub fn new_credentials() -> Result<AuthCredentials> {
    Ok(Credentials::new(
        env::var("CREDENTIALS").unwrap().as_str(),
        &[
            env::var("SECURE_TOKEN_JWK").unwrap().as_str(),
            env::var("SERVICE_ACCOUNT_JWK").unwrap().as_str(),
        ],
    )?)
}

pub fn new_session(credentials: &Credentials, token: Option<String>) -> Result<UserSession> {
    if token.is_none() {
        bail!("HTTPリクエストヘッダに 'authorization' キーが存在しません.");
    }

    let session = Session::by_access_token(credentials, &token.unwrap())?;

    Ok(session)
}
