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
struct RankingScore;

pub async fn ranking_score(graphql_endpoint: &str) -> Result<()> {
    let client = Client::new();
    let req_body = RankingScore::build_query(ranking_score::Variables {});

    let res = client.post(graphql_endpoint).json(&req_body).send().await?;
    let res_body: Response<ranking_score::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
