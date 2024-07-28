use crate::helpers::errors::validation_error_message;
use lettre::address::AddressError;
use serde::Serialize;
use tide::{Response, StatusCode};

// this a custom error enum that abstracts errors

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    // general diesel error
    #[error("{0}")]
    DieselError(#[from] diesel::result::Error),
    // cannot connect to db
    #[error("{0}")]
    ConnectionDBError(#[from] diesel::result::ConnectionError),
    // validation error
    #[error("{}", validation_error_message(.0.clone()))]
    ValidationError(validator::ValidationErrors),
    // for duplicate values in the db like username
    #[error("{0} has already been taken")]
    DuplicateDBError(String),
    // not found in backend
    #[error("{0} not found.")]
    NotFoundError(String),
    // cannot compute hash function in bcrypt for whatever reason
    #[error("{0}")]
    HashError(#[from] bcrypt::BcryptError),
    // password is wrong
    #[error("Incorrect Password")]
    WrongPasswordError(),
    // sessions is wrong
    #[error("Invalid session!")]
    InvalidSessionError(),
    // invalid requests to server
    #[error("{0}")]
    InvalidRequestError(#[from] RequestErrors),
    // invalid response from the server
    #[error("Malformed Response")]
    InvalidResponseError(),
    // anything to do with s3 bucket
    #[error("{0}")]
    S3Error(#[from] S3Errors),
    // anything to do with connection pooling
    #[error("Unable to get connection pool")]
    ConnectionPoolError(),
    // if association to db doesnt work somehow
    #[error("{0}")]
    AssociationError(#[from] AssociationErrors),
    #[error("{0}")]
    EmailError(#[from] lettre::transport::smtp::Error),
    #[error("{0}")]
    AddressError(#[from] AddressError),
    #[error("Wrong password reset code")]
    WrongPasswordResetCodeError(),
    #[error("Could not parse datetime")]
    DatetimeError(),
    #[error("Password reset code expired")]
    PasswordResetCodeExpiredError(),
    #[error("No password reset requested")]
    NoPasswordResetError(),
    #[error("Email already taken")]
    DuplicateEmailError(),
    #[error("Username already taken")]
    DuplicateUsernameError(),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match *self {
            // 5XX errors (These are unchecked)
            Error::DieselError(_) => StatusCode::InternalServerError,
            Error::ConnectionDBError(_) => StatusCode::InternalServerError,
            Error::HashError(_) => StatusCode::InternalServerError,
            Error::InvalidResponseError() => StatusCode::InternalServerError,
            Error::ConnectionPoolError() => StatusCode::InternalServerError,
            Error::EmailError(_) => StatusCode::InternalServerError,
            Error::AddressError(_) => StatusCode::InternalServerError,
            Error::DatetimeError() => StatusCode::InternalServerError,

            // 4XX errors (These are checked)
            Error::ValidationError(_) => StatusCode::BadRequest,
            Error::DuplicateDBError(_) => StatusCode::BadRequest,
            Error::NotFoundError(_) => StatusCode::BadRequest,
            Error::WrongPasswordError() => StatusCode::BadRequest,
            Error::InvalidSessionError() => StatusCode::BadRequest,
            Error::AssociationError(_) => StatusCode::BadRequest,
            Error::S3Error(S3Errors::FailedToDeleteImage) => StatusCode::BadRequest,
            Error::S3Error(S3Errors::FailedToUploadImage) => StatusCode::BadRequest,
            Error::InvalidRequestError(RequestErrors::MalformedParams) => StatusCode::BadRequest,
            Error::InvalidRequestError(RequestErrors::MalformedPayload) => StatusCode::BadRequest,
            Error::WrongPasswordResetCodeError() => StatusCode::BadRequest,
            Error::PasswordResetCodeExpiredError() => StatusCode::BadRequest,
            Error::NoPasswordResetError() => StatusCode::BadRequest,
            Error::DuplicateEmailError() => StatusCode::BadRequest,
            Error::DuplicateUsernameError() => StatusCode::BadRequest,
        }
    }

    pub fn into_response(self) -> tide::Result {
        let status_code = self.get_status_code();

        if status_code == 400 {
            return tide::Body::from_json(&ErrorBody {
                err: self.to_string(),
            })
            .map(|b| Response::builder(status_code).body(b).build())
            .or_else(|e| {
                Err(tide::Error::from_str(
                    StatusCode::InternalServerError,
                    e.to_string(),
                ))
            });
        }
        Err(tide::Error::from_str(status_code, self.to_string()))
    }
}

#[derive(Serialize)]
struct ErrorBody {
    err: String,
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AssociationErrors {
    #[error("Link provided does not belong to the user.")]
    LinkDoesNotBelongToUser,
    #[error("Invalid follow user specified.")]
    InvalidFollowUser,
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum S3Errors {
    #[error("Failed to upload image to s3 bucket.")]
    FailedToUploadImage,
    #[error("Failed to delete image from s3 bucket.")]
    FailedToDeleteImage,
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum RequestErrors {
    #[error("Obtained malformed request payload.")]
    MalformedPayload,
    #[error("Obtained malformed request params.")]
    MalformedParams,
}
