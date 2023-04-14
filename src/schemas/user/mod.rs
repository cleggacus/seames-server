use std::collections::HashSet;

use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHasher};
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use juniper::graphql_object;
use email_address::EmailAddress;

use crate::schema::users;

use self::error::{ValidationError, FieldError};

pub mod query;
pub mod mutation;
pub mod error;

lazy_static::lazy_static! {
    static ref SPECIALS: HashSet<char> = 
        "~`! @#$%^&*()_-+={[}]|\\:;\"'<,>.?/0123456789"
            .chars()
            .collect();
}



#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[graphql_object]
#[graphql(description = "The details for a user")]
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

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        self.validate_email(&mut errors);
        self.validate_password(&mut errors);

        errors
    }

    pub fn hash_password(&mut self) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        self.password = argon2.hash_password(self.password.as_bytes(), &salt).unwrap().to_string();
    }

    fn validate_email(&self, errors: &mut Vec<ValidationError>) {
        if !EmailAddress::is_valid(&self.email) {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "email".into(),
                    message: "invalid email".into(),
                }
            ))

        }
    }

    fn validate_password(&self, errors: &mut Vec<ValidationError>) {
        if self.password.len() < 8 {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "password".into(),
                    message: "must have at least 8 characters".into(),
                }
            ))
        }

        if self.password.len() > 128 {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "password".into(),
                    message: "can't have more than 128 characters".into(),
                }
            ))
        }

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_number_or_special = false;

        for c in self.password.chars() {
            if !has_upper && c.is_uppercase() {
                has_upper = true;
            }

            if !has_lower && c.is_lowercase() {
                has_lower = true;
            }

            if !has_number_or_special && SPECIALS.contains(&c) {
                has_number_or_special = true;
            }
        }

        if !has_upper {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "password".into(),
                    message: "must contain an uppercase character".into(),
                }
            ))
        }

        if !has_lower {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "password".into(),
                    message: "must contain a lowercase character".into(),
                }
            ))
        }

        if !has_number_or_special {
            errors.push(ValidationError::FieldError(
                FieldError {
                    field: "password".into(),
                    message: "must contain a number or special character".into(),
                }
            ))
        }
    }
}

