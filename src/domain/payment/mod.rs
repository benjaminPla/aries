pub mod repository;
pub mod value_objects;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::payment::value_objects::payment_method::PaymentMethod;

pub struct Payment {
    amount_cents:   i32,
    created_at:     DateTime<Utc>,
    id:             Uuid,
    notes:          Option<String>,
    paid_at:        DateTime<Utc>,
    payment_method: PaymentMethod,
    student_id:     Uuid,
}

impl Payment {
    pub fn new(
        amount_cents:   i32,
        notes:          Option<String>,
        paid_at:        DateTime<Utc>,
        payment_method: PaymentMethod,
        student_id:     Uuid
    ) -> Self {
        Self {
            amount_cents,
            created_at:     Utc::now(),
            id:             Uuid::new_v4(),
            notes,
            paid_at,
            payment_method,
            student_id,
        }
    }

    pub fn reconstitute(
        amount_cents:   i32,
        created_at:     DateTime<Utc>,
        id:             Uuid,
        notes:          Option<String>,
        paid_at:        DateTime<Utc>,
        payment_method: PaymentMethod,
        student_id:     Uuid,
    ) -> Self {
        Self {
            amount_cents,
            created_at,
            id,
            notes,
            paid_at,
            payment_method,
            student_id
        }
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn amount_cents(&self)   -> i32           { self.amount_cents }
    pub fn created_at(&self)     -> DateTime<Utc> { self.created_at }
    pub fn id(&self)             -> Uuid          { self.id }
    pub fn notes(&self)          -> Option<&str>  { self.notes.as_deref() }
    pub fn paid_at(&self)        -> DateTime<Utc> { self.paid_at }
    pub fn payment_method(&self) -> &str          { self.payment_method.value() }
    pub fn student_id(&self)     -> Uuid          { self.student_id }
}
