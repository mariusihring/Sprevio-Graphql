
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{GraphiQLSource, MultipartOptions},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::*;
mod graphql;
mod types;

use graphql::{Query, Mutation, Subscription};



pub type SprevioSchema = Schema<Query, Mutation, EmptySubscription>;



async fn index(schema: web::Data<SprevioSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        // .data(Storage::default())
        .finish();



    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        // let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some("pk_test_c3BsZW5kaWQtZG9ua2V5LTE2LmNsZXJrLmFjY291bnRzLmRldiQ".to_string()), None);
        App::new()
            //.wrap(ClerkMiddleware::new(config, None, true))
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(10)),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
