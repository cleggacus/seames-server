use diesel::prelude::*;

use crate::{schema::users, schemas::user::error::UserResult};
use juniper::graphql_object;
use nanoid::nanoid;

use crate::schemas::{user::{NewUser, User}, root::Context};

use super::error::ValidationErrors;

pub struct UserMutation;

#[graphql_object(Context = Context)]
impl UserMutation {
    fn create(context: &Context, email: String, password: String) -> UserResult {
        let conn = &mut context.db_pool.get().unwrap();

        let id = nanoid!();

        let mut new_user = NewUser {
            id,
            email, 
            password
        };

        let errors = new_user.validate();

        if !errors.is_empty() {
            return UserResult::Err(ValidationErrors {
                errors
            })
        }

        new_user.hash_password();

        let result = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
            .unwrap();

        UserResult::Ok(result)
    }
}
