use crate::domain::enrollment::repository::EnrollmentRepoError;

#[derive(Debug, thiserror::Error)]
pub enum EnrollmentAppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("not found")]
    NotFound,
    #[error("el grupo de edad del alumno no coincide con el del curso")]
    AgeGroupMismatch,
    #[error("el curso ha alcanzado su capacidad máxima")]
    CourseFull,
}

impl From<EnrollmentRepoError> for EnrollmentAppError {
    fn from(e: EnrollmentRepoError) -> Self {
        match e {
            EnrollmentRepoError::Database(msg) => Self::Database(msg),
            EnrollmentRepoError::NotFound(_)   => Self::NotFound,
        }
    }
}
