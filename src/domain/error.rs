pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(value: diesel::result::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
