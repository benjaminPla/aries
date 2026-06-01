use crate::domain::payment::repository::PaymentRepoError;

#[derive(Debug, thiserror::Error)]
pub enum PaymentAppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("not found")]
    NotFound,
}

impl From<PaymentRepoError> for PaymentAppError {
    fn from(e: PaymentRepoError) -> Self {
        match e {
            PaymentRepoError::Database(msg) => Self::Database(msg),
            PaymentRepoError::NotFound(_)   => Self::NotFound,
        }
    }
}
