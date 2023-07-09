mod generated;
use clap::Parser;
use generated::{Gql, Query, QueryName};
use graphql_client::{GraphQLQuery, QueryBody};
use serde::Serialize;
use crate::generated::{UserById, user_by_id};
const URL: &str = "http://localhost:3000/graphql";
fn main() -> Result<(), anyhow::Error> {
    let gql = Gql::parse();
    match gql {
        Gql::Query(Query { name }) => {
            let query_body = match name {
                QueryName::UserById { id } => {
                    UserById::build_query(user_by_id::Variables { id })
                }
            };
            run(query_body)?;
        }
        Gql::Mutation(mutation) => {}
    }
    Ok(())
}
pub fn run<T>(query: QueryBody<T>) -> Result<String, anyhow::Error>
where
    T: Serialize,
{
    let client = reqwest::blocking::Client::new();
    let res = client.post(URL).json(&query).send()?;
    let response_body: serde_json::Value = res.json()?;
    println!("{}", response_body);
    Ok(response_body.to_string())
}
