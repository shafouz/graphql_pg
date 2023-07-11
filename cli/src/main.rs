mod generated;
use crate::generated::{create_domain, search_tag_by_value, CreateDomain, SearchTagByValue};
use clap::Parser;
use generated::{Gql, Query, QueryName};
use graphql_client::{GraphQLQuery, QueryBody};
use serde::Serialize;
use serde_json::{Map, Value};
const URL: &str = "http://localhost:3000/graphql";
fn main() -> Result<(), anyhow::Error> {
    let gql = Gql::parse();
    match gql {
        Gql::Query(Query { name }) => {
            match name {
                QueryName::SearchTagByValue { search } => {
                    run(SearchTagByValue::build_query(
                        search_tag_by_value::Variables {
                            search: search.to_string(),
                        },
                    ))?;
                }
                QueryName::CreateDomain { domain, tags } => {
                    run(CreateDomain::build_query(create_domain::Variables {
                        domain: domain.to_string(),
                        tags: vec_to_json_string(tags)?,
                    }))?;
                }
            };
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
pub fn vec_to_json_string(args_vector: Vec<String>) -> Result<String, anyhow::Error> {
    let mut map = Map::new();
    for key_colon_value in args_vector {
        let (k, v) = key_colon_value.split_once(":").unwrap();
        map.insert(k.into(), v.into());
    }
    Ok(Value::Object(map).to_string())
}
