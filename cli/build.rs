use apollo_parser::ast::Definition;
use apollo_parser::Parser;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::fs::read_to_string;
use std::fs::write;

fn main() -> Result<(), anyhow::Error> {
    // let mut code = quote!();
    // let names_variables_and_types = query_attrs("../query.graphql");
    //
    // build_from_schema(&names_variables_and_types, &mut code)?;
    // build_main(&names_variables_and_types)?;

    Ok(())
}

fn build_scalars(code: &mut TokenStream, schema: &str) -> Result<(), anyhow::Error> {
    let query = read_to_string(schema).unwrap();
    let parser = Parser::new(&query);
    let ast = parser.parse();

    for def in ast.document().definitions() {
        if let Definition::ScalarTypeDefinition(scalar) = def {
            let text = scalar.name().unwrap().text();
            let scalar = format_ident!("{}", text.to_string());

            if text.as_str() == "JSON" {
                code.extend(quote! {
                    type #scalar = Vec<String>;
                });
            } else {
                code.extend(quote! {
                    type #scalar = String;
                });
            }
        };
    }

    Ok(())
}

fn build_main(
    names_variables_and_types: &Vec<(String, Vec<(String, String)>)>,
) -> Result<(), anyhow::Error> {
    let url = "http://localhost:3000/graphql";

    let mut names_and_snake_names = vec![];

    let strukts: Vec<_> = names_variables_and_types
        .iter()
        .map(|(name, variables_and_types)| {
            let name = format_ident!("{}", name);
            let snake_name = format_ident!("{}", name.to_string().to_snake_case());

            names_and_snake_names.push(name.clone());
            names_and_snake_names.push(snake_name.clone());

            let variables: Vec<_> = variables_and_types
                .iter()
                .map(|(variable, _)| format_ident!("{}", variable))
                .collect();

            quote! {
                QueryName::#name { #(#variables),* } => {
                    #name::build_query(#snake_name::Variables { #(#variables: #variables.to_string()),* })
                },
            }
        })
        .collect();

    eprintln!("DEBUGPRINT[2]: procs.rs:46: strukts={:#?}", strukts);
    let code = quote! {
        mod generated;

        use clap::Parser;
        use generated::{Gql, Query, QueryName};
        use graphql_client::{GraphQLQuery, QueryBody};
        use serde::Serialize;

        use crate::generated::{#(#names_and_snake_names),*};

        const URL: &str = #url;

        fn main() -> Result<(), anyhow::Error> {
            let gql = Gql::parse();

            match gql {
                Gql::Query(Query { name }) => {
                    let query_body = match name {
                        #(#strukts)*
                    };
                    run(query_body)?;
                },
                Gql::Mutation(mutation) => {
                    // mutations
                }
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
    };

    write("src/main.rs", code.to_string())?;
    Ok(())
}

fn build_from_schema(
    names_variables_and_types: &Vec<(String, Vec<(String, String)>)>,
    code: &mut TokenStream,
) -> Result<(), anyhow::Error> {
    code.extend(quote! {
        use clap_stdin::MaybeStdin;
        use clap::{Parser, Subcommand};
        use graphql_client::{GraphQLQuery};
    });

    build_scalars(code, "../schema.graphql")?;

    for (name, _) in names_variables_and_types {
        let name = format_ident! {
            "{}", name
        };

        code.extend(quote! {
           #[derive(Debug, GraphQLQuery)]
           #[graphql(schema_path = "../schema.graphql", query_path = "../query.graphql")]
           pub struct #name;
        });
    }

    build_args(code, &names_variables_and_types);

    let syntax_tree = syn::parse_file(&code.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    eprintln!("DEBUGPRINT[1]: procs.rs:17: formatted={}", formatted);
    write("src/generated.rs", formatted)?;
    Ok(())
}

fn build_args(
    code: &mut TokenStream,
    names_variables_and_types: &Vec<(String, Vec<(String, String)>)>,
) {
    let strukts: Vec<_> = names_variables_and_types
        .iter()
        .map(|(name, variables_and_types)| {
            let name = format_ident!("{}", name);

            let variables: Vec<_> = variables_and_types
                .iter()
                .map(|(variable, _)| format_ident!("{}", variable))
                .collect();

            let types: Vec<_> = variables_and_types
                .iter()
                .map(|(_, ty)| format_ident!("{}", ty))
                .collect();

            quote! {
                #[derive(Debug, Clone, Subcommand)]
                #[command(rename_all = "snake_case")]
                pub enum QueryName {
                    #name { #(#variables:MaybeStdin<#types>),* }
                }
            }
        })
        .collect();

    code.extend(quote! {
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

        #( #strukts )*
    });
}

fn query_attrs(schema: &str) -> Vec<(String, Vec<(String, String)>)> {
    let query = read_to_string(schema).unwrap();
    let parser = Parser::new(&query);
    let ast = parser.parse();

    let query_names = ast
        .document()
        .definitions()
        .into_iter()
        .filter_map(|def| {
            if let Definition::OperationDefinition(op_def) = def {
                let mut types = vec![];

                for v in op_def
                    .variable_definitions()
                    .iter()
                    .map(|v| v.variable_definitions())
                    .flatten()
                {
                    let v = v.ty().unwrap();

                    match v {
                        apollo_parser::ast::Type::NamedType(t) => {
                            // Option<T>
                            let ty = t.name()?.text().to_string();

                            types.push(
                                match ty.as_str() {
                                    "Int" => "Option<i64>",
                                    "String" => "Option<String>",
                                    "Boolean" => "Option<bool>",
                                    "ID" => "Option<String>",
                                    "Float" => "Option<f64>",
                                    _ => continue,
                                }
                                .to_string(),
                            );
                        }
                        apollo_parser::ast::Type::ListType(_) => {
                            // Vec<T>
                            continue;
                        }
                        apollo_parser::ast::Type::NonNullType(non_null) => {
                            // T
                            let ty = non_null.named_type()?.name()?.text().to_string();

                            types.push(
                                match ty.as_str() {
                                    "Int" => "i64",
                                    "String" => "String",
                                    "Boolean" => "bool",
                                    "ID" => "String",
                                    "Float" => "f64",
                                    _ => continue,
                                }
                                .to_string(),
                            );
                        }
                    };
                }

                let name = op_def
                    .name()
                    .map(|name| name.text().to_string())
                    .unwrap_or_default();
                let variable_definitions = op_def.variable_definitions();

                let variables_and_types: Vec<(String, String)> = variable_definitions
                    .iter()
                    .map(|v| v.variable_definitions())
                    .flatten()
                    .filter_map(|v| Some(v.variable()?.text().to_string()))
                    .zip(types)
                    .collect();

                Some((name, variables_and_types))
            } else {
                None
            }
        })
        .collect::<Vec<(String, Vec<(String, String)>)>>();

    query_names
}
