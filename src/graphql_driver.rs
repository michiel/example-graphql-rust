use actix::prelude::*;
use futures::future::Future;
use serde_json;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use std;
use super::{AppState, DBPool};

use actix_web::{http, HttpRequest, HttpResponse,
                HttpMessage, AsyncResponder, Error};
use graphql_schema::{Schema, create_schema};

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    pub schema: std::sync::Arc<Schema>,
    pub db_pool: DBPool
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn graphiql(_req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("/graphql");
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
    )
}

pub fn graphql(req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let executor = req.state().executor.clone();
    req.json()
        .from_err()
        .and_then(move |val: GraphQLData| {
            executor.send(val).from_err().and_then(|res| match res {
                Ok(user) => Ok(
                    HttpResponse::Ok()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .body(user),
                ),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
        })
        .responder()
}

pub fn create_executor(pool: DBPool) -> Addr<Syn, GraphQLExecutor> {
    SyncArbiter::start(3, move || GraphQLExecutor {
        schema: std::sync::Arc::new(create_schema()),
        db_pool: pool.clone()
    })
}

