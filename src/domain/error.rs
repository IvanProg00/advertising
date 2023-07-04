use thiserror::Error;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Not found")]
    NotFound,
    #[error("Undefined error")]
    UndefinedError,
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::UndefinedError,
        }
    }
}

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Not found")]
    NotFound,
    #[error("Undefined error")]
    UndefinedError,
}

impl From<RepositoryError> for CommonError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::UndefinedError => Self::UndefinedError,
        }
    }
}
