use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::teacher::Teacher;

#[derive(Clone)]
pub struct TeacherDto {
    pub id:         Uuid,
    pub email:      String,
    pub first_name: String,
    pub last_name:  String,
    pub notes:      Option<String>,
    pub phone:      String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Teacher> for TeacherDto {
    fn from(t: &Teacher) -> Self {
        Self {
            id:         t.id(),
            email:      t.email().value().to_owned(),
            first_name: t.first_name().value().to_owned(),
            last_name:  t.last_name().value().to_owned(),
            notes:      t.notes().map(str::to_owned),
            phone:      t.phone().value().to_owned(),
            created_at: t.created_at(),
            updated_at: t.updated_at(),
        }
    }
}
