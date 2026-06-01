use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::payment::{dto::PaymentDto, errors::PaymentAppError},
    domain::payment::repository::PaymentRepo,
};

pub struct PaymentGetByEnrollmentUseCase {
    payment_repo: Arc<dyn PaymentRepo>,
}

impl PaymentGetByEnrollmentUseCase {
    pub fn new(payment_repo: Arc<dyn PaymentRepo>) -> Self { Self { payment_repo } }

    pub fn execute(&self, enrollment_id: Uuid) -> Result<Vec<PaymentDto>, PaymentAppError> {
        let payments = self.payment_repo.get_by_enrollment(enrollment_id)?;
        Ok(payments.iter().map(PaymentDto::from).collect())
    }
}
