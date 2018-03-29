//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor that use diesel.
//! Technically sync actors are worker style actors, multiple of them
//! can run in parallel and process messages from same queue.
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate uuid;
extern crate futures;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate juniper;

use actix::prelude::*;
use actix_web::{http, server, middleware,
                App, Path, State, HttpRequest, HttpResponse, HttpMessage, AsyncResponder, FutureResponse, Error
};

use diesel::prelude::*;
use diesel::r2d2::{ Pool, ConnectionManager };
use futures::future::Future;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db;
mod models;
mod database_schema;
mod graphql_schema;

use db::{CreateUser, DbExecutor};

use graphql_schema::{Schema, create_schema};

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: std::sync::Arc<Schema>
}

impl GraphQLExecutor {
    fn new(schema: std::sync::Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor {
            schema: schema,
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

fn graphiql(_req: HttpRequest<AppState>) -> Result<HttpResponse, Error>  {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql(req: HttpRequest<AppState>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let executor = req.state().executor.clone();
    req.json()
        .from_err()
        .and_then(move |val: GraphQLData| {
            executor.send(val)
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(user) => Ok(HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").body(user)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into())
                    }
                })
        })
        .responder()
}
/// State with DbExecutor address
struct AppState {
    db: Addr<Syn, DbExecutor>,
    executor: Addr<Syn, GraphQLExecutor>,
}

/// Async request handler
fn index(name: Path<String>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state.db.send(CreateUser{name: name.into_inner()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-example");

    // Start 3 db executor actors
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let db_addr = SyncArbiter::start(3, move || {
        DbExecutor(pool.clone())
    });

    let schema = std::sync::Arc::new(create_schema());
    let gq_addr = SyncArbiter::start(3, move || {
        GraphQLExecutor::new(schema.clone())
    });

    // Start http server
    server::new(move || {
        App::with_state(AppState{
            db: db_addr.clone(),
            executor: gq_addr.clone(),
        })
        // enable logger
        .middleware(middleware::Logger::default())
            .resource("/{name}", |r| r.method(http::Method::GET).with2(index))
            .resource("/graphql", |r| r.method(http::Method::POST).h(graphql))
            .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
    })
    .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
