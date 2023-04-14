use std::sync::RwLock;

use actix_web::{get, route, web, Error, HttpResponse, Responder, cookie::CookieJar, HttpRequest};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest, playground::playground_source};

use crate::{db::DBPool, schemas::root::{Schema, Context, create_schema}};



#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    req: HttpRequest,
    pool: web::Data<DBPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let mut jar = CookieJar::new();

    let cookies = req.cookies();

    if let Ok(cookies) = cookies {
        for cookie in cookies.iter() {
            jar.add_original(cookie.clone());
        }
    }

    let ctx = Context {
        cookie_jar: RwLock::new(jar),
        db_pool: pool.get_ref().to_owned(),
    };


    let res = data.execute(&schema, &ctx).await;
    let mut http_response = HttpResponse::Ok();

    let jar = ctx.cookie_jar.write().unwrap();

    for cookie in jar.delta() {
        http_response.cookie(cookie.clone());
    }

    Ok(http_response.json(res))
}

#[get("/graphiql")]
async fn graphiql() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[get("/playground")]
async fn playground() -> impl Responder {
    Html(playground_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(graphql)
        .service(playground)
        .service(graphiql);
}
