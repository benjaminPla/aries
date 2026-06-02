#[derive(Debug, Clone)]
pub struct LastName(String);

impl LastName {
    const MAX_CHARS: usize = 50;
    const MIN_CHARS: usize = 3;

    pub fn new(value: impl Into<String>) -> Result<Self, LastNameError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(LastNameError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(LastNameError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(LastNameError::TooShort(Self::MIN_CHARS)) }
        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum LastNameError {
    #[error("el apellido no puede estar vacío")]
    Empty,
    #[error("el apellido no puede tener más de {0} caracteres")]
    TooLong(usize),
    #[error("el apellido no puede tener menos de {0} caracteres")]
    TooShort(usize),
}
