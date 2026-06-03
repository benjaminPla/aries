use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::course_period::CoursePeriod;

#[derive(Clone)]
pub struct CoursePeriodDto {
    pub id:         Uuid,
    pub course_id:  Uuid,
    pub label:      String,
    pub start_date: NaiveDate,
    pub end_date:   NaiveDate,
    pub enrolled:   i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CoursePeriodDto {
    pub fn is_active(&self) -> bool {
        self.end_date >= chrono::Local::now().date_naive()
    }
}

impl From<&CoursePeriod> for CoursePeriodDto {
    fn from(p: &CoursePeriod) -> Self {
        Self {
            id:         p.id(),
            course_id:  p.course_id(),
            label:      p.label().to_owned(),
            start_date: p.start_date(),
            end_date:   p.end_date(),
            enrolled:   p.enrolled(),
            created_at: p.created_at(),
            updated_at: p.updated_at(),
        }
    }
}
