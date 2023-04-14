use juniper::{GraphQLObject, GraphQLUnion};

use super::User;

#[derive(GraphQLObject)]
pub struct FieldError {
    pub field: String,
    pub message: String
}

#[derive(GraphQLObject)]
pub struct GeneralError {
    pub message: String
}

#[derive(GraphQLUnion)]
pub enum ValidationError {
    FieldError(FieldError),
    Unauthorized(GeneralError),
}

#[derive(GraphQLObject)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>
}

#[derive(GraphQLUnion)]
pub enum UserResult {
    Ok(User), 
    Err(ValidationErrors)
}

