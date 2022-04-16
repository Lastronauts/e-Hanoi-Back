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
struct CreateUserInDB;

pub async fn create_user_in_db(graphql_endpoint: &str, token: &str) -> Result<()> {
    let input = create_user_in_db::Variables {
        new_user: create_user_in_db::NewUser {
            name: String::from("hoge"),
        },
    };

    let client = Client::new();
    let req_body = CreateUserInDB::build_query(input);

    let res = client
        .post(graphql_endpoint)
        .header(header::AUTHORIZATION, token)
        .json(&req_body)
        .send()
        .await?;
    let res_body: Response<create_user_in_db::ResponseData> = res.json().await?;
    println!("{:#?}", res_body);

    Ok(())
}
