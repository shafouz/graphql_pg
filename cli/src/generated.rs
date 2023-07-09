use clap::{Parser, Subcommand};
use graphql_client::GraphQLQuery;
#[derive(Debug, GraphQLQuery)]
#[graphql(schema_path = "../schema.graphql", query_path = "../query.graphql")]
pub struct UserById;
#[derive(Parser)]
#[command(name = "gql")]
#[command(bin_name = "gql")]
pub enum Gql {
    Query(Query),
    Mutation(Mutation),
}
#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Query {
    #[command(subcommand)]
    pub name: QueryName,
}
#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Mutation {}
#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "snake_case")]
pub enum QueryName {
    UserById { id: i64 },
}
