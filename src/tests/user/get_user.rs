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
    response_derives = "Debug"
)]
struct GetUser;

pub async fn get_user(graphql_endpoint: &String) -> Result<()> {
    let test_token = get_test_token()?;

    let client = Client::new();
    let req_body = GetUser::build_query(get_user::Variables {});

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, test_token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<get_user::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
