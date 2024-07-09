
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use rocket::{response::content, routes, State};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use async_graphql::*;
mod graphql;
mod types;

use graphql::{Query, Mutation, Subscription};



pub type SprevioSchema = Schema<Query, Mutation, EmptySubscription>;


#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<SprevioSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<SprevioSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::launch]
fn rocket() -> _ {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

    rocket::build().manage(schema).mount("/", routes![graphql_query, graphql_request, graphiql])
}

