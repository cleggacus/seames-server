use std::sync::RwLock;

use actix_web::cookie::CookieJar;
use juniper::{graphql_object, RootNode, EmptySubscription};

use crate::{db::DBPool, schemas::user::mutation::UserMutation};

use super::user::query::UserQuery;

pub struct Context {
    pub cookie_jar: RwLock<CookieJar>,
    pub db_pool: DBPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn user() -> UserQuery { 
        UserQuery 
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn user() -> UserMutation { 
        UserMutation 
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
