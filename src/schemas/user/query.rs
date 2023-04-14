use actix_web::cookie::Cookie;
use argon2::{Argon2, PasswordVerifier, PasswordHash};
use diesel::prelude::*;
use juniper::graphql_object;

use crate::{schemas::root::Context, utils::auth::{update_token_cookies, get_authed_user}};

use super::User;

pub struct UserQuery;

#[graphql_object(Context = Context)]
impl UserQuery {
    fn find(context: &Context, id: String) -> Option<User> {
        use crate::schema::users::dsl::{users, id as user_id};

        let conn = &mut context.db_pool.get().unwrap();

        let user = users
            .filter(user_id.eq(id))
            .get_result(conn);

        match user {
            Ok(user) => Some(user),
            Err(_) => None
        }
    }

    fn me(context: &Context) -> Option<User> {
        get_authed_user(context)
    }

    fn signout(context: &Context) -> bool {
        let jar = &mut context.cookie_jar.write().unwrap();

        jar.remove(Cookie::named("refresh_token"));
        jar.remove(Cookie::named("access_token"));

        true
    }

    fn signin(context: &Context, email: String, password: String) -> Option<User> {
        use crate::schema::users::dsl::{users, email as user_email};

        let conn = &mut context.db_pool.get().unwrap();
        let jar = &mut context.cookie_jar.write().unwrap();

        let user = users
            .filter(user_email.eq(email))
            .get_result::<User>(conn);

        match user {
            Ok(user) => {
                let argon2 = Argon2::default();
                let parsed_hash = PasswordHash::new(&user.password).unwrap();
                let is_correct = argon2.verify_password(password.as_bytes(), &parsed_hash);

                if is_correct.is_ok() {
                    update_token_cookies(&user, jar);
                    return Some(user);
                }

                None
            },
            Err(_) => None
        }

    }
}
