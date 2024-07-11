use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::iter::Map;
use std::sync::{Arc, Mutex};
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{GraphiQLSource, MultipartOptions},
    EmptySubscription, Schema,
};
use surrealdb::Surreal;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::*;
use surrealdb::engine::any;
use surrealdb::engine::remote::ws::{Client, Ws};

mod graphql;
mod types;
mod database;

use graphql::{Query, Mutation};

type DbConnection = Surreal<Client>;
type SurrealUserTokens = Arc<Mutex<BTreeMap<String, String>>>;



struct GraphQlContext {
    db: DbConnection
}
async fn create_context(db: DbConnection) -> GraphQlContext {
    GraphQlContext { db }
}

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
    let user_tokens: SurrealUserTokens = Arc::new(Mutex::new(BTreeMap::new()));
    let db: DbConnection = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Failed to connect to surreal_Db");
    db.use_db("test").await.expect("failed to use db test");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(user_tokens)
        .finish();


    // Export SDL
    let mut file = File::create("schema.graphqls")?;
    file.write_all(&schema.sdl().as_bytes())?;


    println!("GraphiQL IDE: http://localhost:8001");

    HttpServer::new(move || {
        // let config: ClerkConfiguration = ClerkConfiguration::new(None, None, "".to_string()), None);
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
        .bind("127.0.0.1:8001")?
        .run()
        .await
}
