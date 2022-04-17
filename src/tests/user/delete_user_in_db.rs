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
struct DeleteUserInDB;

pub async fn delete_user_in_db(graphql_endpoint: &str, token: &str) -> Result<()> {
    let client = Client::new();
    let req_body = DeleteUserInDB::build_query(delete_user_in_db::Variables {});

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<delete_user_in_db::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
