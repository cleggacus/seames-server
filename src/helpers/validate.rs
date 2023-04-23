use std::collections::HashSet;

use email_address::EmailAddress;

use super::errors::{FieldErrors, FieldError};

lazy_static::lazy_static! {
    static ref SPECIALS: HashSet<char> = 
        "~`! @#$%^&*()_-+={[}]|\\:;\"'<,>.?/0123456789"
            .chars()
            .collect();
}

pub struct Validate;

impl Validate {
    pub fn email(field: &str, email: &str, errors: &mut FieldErrors) {
        if !EmailAddress::is_valid(email) {
            errors.push(FieldError::new(field, "invalid email"));
        }
    }

    pub fn password(field: &str, password: &str, errors: &mut FieldErrors) {
        if password.len() < 8 {
            errors.push(FieldError::new(field, "must have at least 8 characters"));
        }

        if password.len() > 128 {
            errors.push(FieldError::new(field, "can't have more than 128 characters"));
        }

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_number_or_special = false;

        for c in password.chars() {
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
            errors.push(FieldError::new(field, "must contain an uppercase character"));
        }

        if !has_lower {
            errors.push(FieldError::new(field, "must contain an lowercase character"));
        }

        if !has_number_or_special {
            errors.push(FieldError::new(field, "must contain a number or special character"));
        }
    }
}

