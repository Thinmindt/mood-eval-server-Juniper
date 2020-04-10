#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;
extern crate chrono;
#[macro_use]
extern crate diesel;

pub mod model;
pub mod schema_diesel;
mod schema;

use juniper::{EmptyMutation, RootNode};
use model::Database;
use rocket::response::content;
use rocket::State;
use schema::Query;

use model::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

type Schema = RootNode<'static, Query, EmptyMutation<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema_diesel::day_data::dsl::*;

    let connection = establish_connection();
    let results = day_data.limit(5)
        .load::<DayData>(&connection)
        .expect("Error loading day data");

    println!("Displaying {} data", results.len());
    for result_day in results {
        println!("{}", result_day.day);
        println!("{}", result_day.mood);
        println!("--------------\n");
    }

    rocket::ignite()
        //.manage(Database::new())
        .manage(Schema::new(Query, EmptyMutation::<Database>::new()))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_connection() {
        assert!(true);
    }
}