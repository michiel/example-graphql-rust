//! Actix web diesel example
//!
extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate juniper;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path,
                State};

use futures::future::Future;

mod models;
mod database_driver;
mod database_schema;
mod database_queries;
mod graphql_driver;
mod graphql_schema;

use database_driver::{get_db_connection_pool, CreateUser, DBPool, DbExecutor};
use graphql_driver::GraphQLExecutor;

pub struct AppState {
    db: Addr<Syn, DbExecutor>,
    executor: Addr<Syn, GraphQLExecutor>,
}

/// Async request handler
fn index(name: Path<String>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let sys = actix::System::new("xds");

    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    // Start http server
    server::new(move || {
        App::with_state(
            AppState{
                db: database_driver::get_db_address(),
                executor: graphql_driver::create_executor(get_db_connection_pool())
        })
        // enable logger
        .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).h(graphql_driver::graphql))
            .resource("/graphiql", |r| r.method(http::Method::GET).h(graphql_driver::graphiql))
            .resource("/get/{name}", |r| r.method(http::Method::GET).with2(index))
    }).bind(&server_port)
        .unwrap()
        .start();

    println!("Started http server: {}", server_port);
    let _ = sys.run();
}
