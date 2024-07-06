use tide::StatusCode;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("{0}")]
    ConnectionDBError(#[from] diesel::result::ConnectionError),
    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("{0} already exists")]
    DuplicateDBError(String), // for duplicate values in the db like username
    #[error("{0} not found.")]
    NotFoundError(String),
    #[error("{0}")]
    HashError(#[from] bcrypt::BcryptError),
    #[error("Incorrect Password")]
    WrongPasswordError(),
    #[error("Invalid session!")]
    InvalidSessionError(),
    #[error("Malformed request payload.")]
    InvalidRequestError(),
    #[error("Malformed Response")]
    InvalidResponseError(),
    #[error("{0}")]
    UploadImageError(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match *self {
            // 5XX errors (These are unchecked)
            Error::DieselError(_) => StatusCode::InternalServerError,
            Error::ConnectionDBError(_) => StatusCode::InternalServerError,
            Error::HashError(_) => StatusCode::InternalServerError,
            Error::InvalidResponseError() => StatusCode::InternalServerError,

            // 4XX errors (These are checked)
            Error::ValidationError(_) => StatusCode::BadRequest,
            Error::DuplicateDBError(_) => StatusCode::BadRequest,
            Error::NotFoundError(_) => StatusCode::BadRequest,
            Error::WrongPasswordError() => StatusCode::BadRequest,
            Error::InvalidSessionError() => StatusCode::BadRequest,
            Error::UploadImageError(_) => StatusCode::BadRequest,
            Error::InvalidRequestError() => StatusCode::BadRequest,
        }
    }

    fn get_client_message(self) -> String {
        let status_code = self.get_status_code();
        // internal server error should not expose internal workings to client
        if status_code == 500 {
            return "".to_string();
        }
        // 400 200 can
        self.to_string()
    }

    pub fn into_response(self) -> tide::Result {
        Err(tide::Error::from_str(
            self.get_status_code(),
            self.get_client_message(),
        ))
    }
}
