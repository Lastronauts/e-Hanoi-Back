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
struct ListUser;

pub async fn list_user(graphql_endpoint: &String) -> Result<()> {
    let input = list_user::Variables {
        name: String::from("yukarisann-lover"),
        start: None,
        range: Some(100),
    };

    let client = Client::new();
    let req_body = ListUser::build_query(input);

    let res = client.post(graphql_endpoint).json(&req_body).send().await?;
    let res_body: Response<list_user::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
