//! Actix web diesel example
//!
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate uuid;
extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate juniper;
extern crate dotenv;

use actix::prelude::*;
use actix_web::{http, server, middleware, App, Path, State, HttpRequest, HttpResponse,
                HttpMessage, AsyncResponder, FutureResponse, Error};

use futures::future::Future;

mod models;
mod database_driver;
mod database_schema;
mod graphql_driver;
mod graphql_schema;

use database_driver::{CreateUser, DbExecutor, DBPool, get_db_connection_pool};
use graphql_driver::{GraphQLExecutor};
use graphql_schema::{create_schema};

pub struct AppState {
    db: Addr<Syn, DbExecutor>,
    executor: Addr<Syn, GraphQLExecutor>,
}

/// Async request handler
fn index(name: Path<String>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser { name: name.into_inner() })
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
    let sys = actix::System::new("xds");

    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    // Start http server
    server::new(move || {
        App::with_state(
            AppState{
                db: database_driver::get_db_address(),
                executor: graphql_driver::create_executor(database_driver::get_db_address())
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
