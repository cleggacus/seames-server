use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng}, PasswordHasher, PasswordVerifier, PasswordHash};
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, prelude::*, result::{Error::{NotFound, DatabaseError}, DatabaseErrorKind::UniqueViolation}};
use juniper::graphql_object;
use nanoid::nanoid;

use crate::{schema::users, db::DBPooledConnection, helpers::errors::ErrorCode, validation_result};

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[graphql_object(
    name = "User",
    description = "User model",
)]
impl User {
    #[graphql(description = "The users ID in base64 format")]
    fn id(&self) -> &str {
        &self.id
    }

    #[graphql(description = "The users email")]
    fn email(&self) -> &str {
        &self.email
    }

    #[graphql(description = "DateTime for when the user was created")]
    fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    #[graphql(description = "DateTime for when the user was last updated")]
    fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

validation_result!(UserResult, User);

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(email: &str, password: &str) -> NewUser {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();

        NewUser { 
            id: nanoid!(),
            email: email.into(),
            password: password_hash,
        }
    }
}

pub struct UserOperation;

impl UserOperation {
    pub fn create(conn: &mut DBPooledConnection, email: &str, password: & str) -> UserResult {
        let new_user = NewUser::new(email, password);

        let result = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn);

        match result {
            Ok(user) => UserResult::User(user),
            Err(DatabaseError(UniqueViolation, _)) =>
                UserResult::conflict("user already exists"),
            Err(_) => UserResult::server(),
        }
    }

    pub fn auth(conn: &mut DBPooledConnection, email: &str, password: &str) -> UserResult {
        use crate::schema::users::dsl::{users, email as user_email};
        
        let user = users
            .filter(user_email.eq(email))
            .get_result::<User>(conn);

        match user {
            Ok(user) => {
                let argon2 = Argon2::default();
                let parsed_hash = PasswordHash::new(&user.password).unwrap();

                let verified = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();

                if verified {
                    return UserResult::User(user);
                }

                return UserResult::unauthorized("auth failed");
            },
            Err(NotFound) => UserResult::unauthorized("auth failed"),
            Err(_) => UserResult::server(),
        }
    }

    pub fn find(conn: &mut DBPooledConnection, id: &str) -> UserResult {
        use crate::schema::users::dsl::{users, id as user_id};
        
        let user = users
            .filter(user_id.eq(id))
            .get_result::<User>(conn);

        match user {
            Ok(user) => UserResult::User(user),
            Err(NotFound) => UserResult::not_found("user not found"),
            Err(_) => UserResult::server(),
        }
    }
}

