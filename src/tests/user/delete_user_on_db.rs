use super::get_test_token;
use actix_web::http::header;
use anyhow::Result;
use graphql_client::{
    GraphQLQuery,
    Response,
};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/schema.graphql",
    query_path = "src/tests/user/query.graphql",
    response_derives = "Debug",
)]
struct DeleteUserOnDb;

pub async fn delete_user_on_db(graphql_endpoint: &String) -> Result<()> {
    let test_token = get_test_token()?;

    let client = Client::new();
    let req_body = DeleteUserOnDb::build_query(delete_user_on_db::Variables {});

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, test_token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<delete_user_on_db::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
