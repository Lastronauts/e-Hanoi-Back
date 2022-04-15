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
struct CreateScore;

pub async fn create_score(graphql_endpoint: &str, token: &str) -> Result<()> {
    let input = create_score::Variables {
        new_score: create_score::NewScore { clearTime: 114 },
    };

    let client = Client::new();
    let req_body = CreateScore::build_query(input);

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<create_score::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
