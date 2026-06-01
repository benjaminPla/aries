use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::enrollment::{dto::EnrollmentDto, errors::EnrollmentAppError},
    domain::enrollment::repository::EnrollmentRepo,
};

pub struct EnrollmentGetByCourseUseCase {
    enrollment_repo: Arc<dyn EnrollmentRepo>,
}

impl EnrollmentGetByCourseUseCase {
    pub fn new(enrollment_repo: Arc<dyn EnrollmentRepo>) -> Self { Self { enrollment_repo } }

    pub fn execute(&self, course_id: Uuid) -> Result<Vec<EnrollmentDto>, EnrollmentAppError> {
        let enrollments = self.enrollment_repo.get_by_course(course_id)?;
        Ok(enrollments.iter().map(EnrollmentDto::from).collect())
    }
}
