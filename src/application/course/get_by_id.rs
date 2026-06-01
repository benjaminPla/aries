use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::course::{dto::CourseDto, errors::CourseAppError},
    domain::course::repository::CourseRepo,
};

pub struct CourseGetByIdUseCase {
    course_repo: Arc<dyn CourseRepo>,
}

impl CourseGetByIdUseCase {
    pub fn new(course_repo: Arc<dyn CourseRepo>) -> Self { Self { course_repo } }

    pub fn execute(&self, id: Uuid) -> Result<CourseDto, CourseAppError> {
        let course = self.course_repo.get_by_id(id)?;
        Ok(CourseDto::from(&course))
    }
}
