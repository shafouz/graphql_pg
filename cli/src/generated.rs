use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;
use graphql_client::GraphQLQuery;
type Cursor = String;
type JSON = String;
type Time = String;
#[derive(Debug, GraphQLQuery)]
#[graphql(schema_path = "../schema.graphql", query_path = "../query.graphql")]
pub struct SearchTagByValue;
#[derive(Debug, GraphQLQuery)]
#[graphql(schema_path = "../schema.graphql", query_path = "../query.graphql")]
pub struct CreateDomain;
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
    SearchTagByValue {
        search: MaybeStdin<String>,
    },
    CreateDomain {
        domain: MaybeStdin<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
}
