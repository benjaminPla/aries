use crate::domain::course::repository::CourseRepoError;

#[derive(Debug, thiserror::Error)]
pub enum CourseAppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("not found")]
    NotFound,
}

impl From<CourseRepoError> for CourseAppError {
    fn from(e: CourseRepoError) -> Self {
        match e {
            CourseRepoError::Database(msg) => Self::Database(msg),
            CourseRepoError::NotFound(_)   => Self::NotFound,
        }
    }
}
