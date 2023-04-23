use juniper::{GraphQLObject, GraphQLEnum};

#[derive(GraphQLEnum)]
pub enum ErrorCode {
    NotFound,
    Unauthorized,
    Conflict,
    ServerError
}

#[derive(GraphQLObject)]
pub struct FieldError {
    pub field: String,
    pub message: String
}

impl FieldError {
    pub fn new(field: &str, message: &str) -> FieldError {
        FieldError {
            field: field.into(),
            message: message.into()
        }
    }
}

#[derive(GraphQLObject)]
pub struct FieldErrors {
    pub errors: Vec<FieldError>,
}

impl FieldErrors {
    pub fn new() -> FieldErrors {
        FieldErrors {
            errors: Vec::new()
        }
    }

    pub fn push(&mut self, error: FieldError) -> &mut Self {
        self.errors.push(error);
        self
    }

    pub fn empty(&self) -> bool {
        self.errors.is_empty()
    }
}

#[derive(GraphQLObject)]
pub struct GeneralError {
    pub code: ErrorCode,
    pub message: String
}


#[macro_export]
macro_rules! validation_result {
    ($name:ident, $for:ident) => {
        #[derive(juniper::GraphQLUnion)]
        pub enum $name {
            $for($for),
            FieldErrors(crate::helpers::errors::FieldErrors),
            GeneralError(crate::helpers::errors::GeneralError),
        }

        impl $name {
            pub fn not_found(message: &str) -> $name {
                $name::GeneralError(crate::helpers::errors::GeneralError {
                    code: ErrorCode::NotFound,
                    message: message.into()
                })
            }

            pub fn unauthorized(message: &str) -> $name {
                $name::GeneralError(crate::helpers::errors::GeneralError {
                    code: ErrorCode::Unauthorized,
                    message: message.into()
                })
            }

            pub fn conflict(message: &str) -> $name {
                $name::GeneralError(crate::helpers::errors::GeneralError {
                    code: ErrorCode::Conflict,
                    message: message.into()
                })
            }

            pub fn server() -> $name {
                $name::GeneralError(crate::helpers::errors::GeneralError {
                    code: ErrorCode::ServerError,
                    message: "something went wrong".into()
                })
            }
        }
    };
}

