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
    query_path = "src/tests/score/query.graphql",
    response_derives = "Debug"
)]
struct DeleteScore;

pub async fn delete_score(graphql_endpoint: &str, token: &str) -> Result<()> {
    let client = Client::new();
    let req_body = DeleteScore::build_query(delete_score::Variables { id: 0 });

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<delete_score::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
