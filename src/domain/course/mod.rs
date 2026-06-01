pub mod repository;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::shared::value_objects::age_group::AgeGroup;

pub struct Course {
    id:           Uuid,
    teacher_id:   Uuid,
    teacher_name: String,
    name:         String,
    age_group:    AgeGroup,
    capacity:     i16,
    price_cents:  i32,
    enrolled:     i64,
    notes:        Option<String>,
    created_at:   DateTime<Utc>,
    updated_at:   DateTime<Utc>,
}

impl Course {
    pub fn new(
        teacher_id:  Uuid,
        name:        String,
        age_group:   AgeGroup,
        capacity:    i16,
        price_cents: i32,
        notes:       Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id:           Uuid::new_v4(),
            teacher_id,
            teacher_name: String::new(),
            name,
            age_group,
            capacity,
            price_cents,
            enrolled:     0,
            notes,
            created_at:   now,
            updated_at:   now,
        }
    }

    pub fn reconstitute(
        id:           Uuid,
        teacher_id:   Uuid,
        teacher_name: String,
        name:         String,
        age_group:    AgeGroup,
        capacity:     i16,
        price_cents:  i32,
        enrolled:     i64,
        notes:        Option<String>,
        created_at:   DateTime<Utc>,
        updated_at:   DateTime<Utc>,
    ) -> Self {
        Self { id, teacher_id, teacher_name, name, age_group, capacity, price_cents, enrolled, notes, created_at, updated_at }
    }

    // ── Mutations ────────────────────────────────────────────────────────────

    pub fn update(
        &mut self,
        teacher_id:  Uuid,
        name:        String,
        age_group:   AgeGroup,
        capacity:    i16,
        price_cents: i32,
        notes:       Option<String>,
    ) {
        self.teacher_id  = teacher_id;
        self.name        = name;
        self.age_group   = age_group;
        self.capacity    = capacity;
        self.price_cents = price_cents;
        self.notes       = notes;
    }

    // ── Getters ──────────────────────────────────────────────────────────────

    pub fn id(&self)           -> Uuid            { self.id }
    pub fn teacher_id(&self)   -> Uuid            { self.teacher_id }
    pub fn teacher_name(&self) -> &str            { &self.teacher_name }
    pub fn name(&self)         -> &str            { &self.name }
    pub fn age_group(&self)    -> &AgeGroup       { &self.age_group }
    pub fn capacity(&self)     -> i16             { self.capacity }
    pub fn price_cents(&self)  -> i32             { self.price_cents }
    pub fn enrolled(&self)     -> i64             { self.enrolled }
    pub fn notes(&self)        -> Option<&str>    { self.notes.as_deref() }
    pub fn created_at(&self)   -> DateTime<Utc>   { self.created_at }
    pub fn updated_at(&self)   -> DateTime<Utc>   { self.updated_at }
}
