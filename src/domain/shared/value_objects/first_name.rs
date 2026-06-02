#[derive(Debug, Clone)]
pub struct FirstName(String);

impl FirstName {
    const MAX_CHARS: usize = 50;
    const MIN_CHARS: usize = 3;

    pub fn new(value: impl Into<String>) -> Result<Self, FirstNameError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(FirstNameError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(FirstNameError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(FirstNameError::TooShort(Self::MIN_CHARS)) }
        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum FirstNameError {
    #[error("el nombre no puede estar vacío")]
    Empty,
    #[error("el nombre no puede tener más de {0} caracteres")]
    TooLong(usize),
    #[error("el nombre no puede tener menos de {0} caracteres")]
    TooShort(usize),
}

