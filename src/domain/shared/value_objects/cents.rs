#[derive(Debug, Clone, Copy)]
pub struct Cents(i32);

impl Cents {
    pub fn new(value: i32) -> Result<Self, CentsError> {
        if value <= 0 { return Err(CentsError::NonPositive) }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 { self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum CentsError {
    #[error("el monto debe ser mayor a 0")]
    NonPositive,
}
