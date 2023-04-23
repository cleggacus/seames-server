use std::sync::RwLock;

use actix_web::cookie::CookieJar;
use juniper::{graphql_object, RootNode, EmptySubscription};

use crate::{db::DBPool, models::user::{UserResult, UserOperation}, helpers::{validate::Validate, errors::FieldErrors, auth::{set_authed_user, get_authed_user}}};

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

    fn me(context: &Context) -> UserResult {
        let mut conn = context.db_pool.get().unwrap();
        let mut jar = context.cookie_jar.write().unwrap();

        get_authed_user(&mut conn, &mut jar)
    }

    fn signIn(context: &Context, email: String, password: String) -> UserResult {
        let mut conn = context.db_pool.get().unwrap();
        let user = UserOperation::auth(&mut conn, &email, &password);

        if let UserResult::User(user) = &user {
            let mut jar = context.cookie_jar.write().unwrap();
            set_authed_user(user, &mut jar);
        }

        user
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    fn createUser(context: &Context, email: String, password: String) -> UserResult {
        let mut errors = FieldErrors::new();

        Validate::email("email", &email, &mut errors);
        Validate::password("password", &password, &mut errors);

        if !errors.empty() {
            return UserResult::FieldErrors(errors);
        }

        let mut conn = context.db_pool.get().unwrap();
        UserOperation::create(&mut conn, &email, &password)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
