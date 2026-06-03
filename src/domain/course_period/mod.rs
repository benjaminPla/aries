pub mod repository;

use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

pub struct CoursePeriod {
    id:         Uuid,
    course_id:  Uuid,
    label:      String,
    start_date: NaiveDate,
    end_date:   NaiveDate,
    enrolled:   i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CoursePeriod {
    pub fn new(course_id: Uuid, label: String, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            course_id,
            label,
            start_date,
            end_date,
            enrolled: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn reconstitute(
        id:         Uuid,
        course_id:  Uuid,
        label:      String,
        start_date: NaiveDate,
        end_date:   NaiveDate,
        enrolled:   i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self { id, course_id, label, start_date, end_date, enrolled, created_at, updated_at }
    }

    pub fn is_active(&self) -> bool {
        self.end_date >= chrono::Local::now().date_naive()
    }

    pub fn id(&self)         -> Uuid          { self.id }
    pub fn course_id(&self)  -> Uuid          { self.course_id }
    pub fn label(&self)      -> &str          { &self.label }
    pub fn start_date(&self) -> NaiveDate     { self.start_date }
    pub fn end_date(&self)   -> NaiveDate     { self.end_date }
    pub fn enrolled(&self)   -> i64           { self.enrolled }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
}
