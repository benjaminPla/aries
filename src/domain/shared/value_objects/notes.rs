#[derive(Debug, Clone)]
pub struct Notes(String);

impl Notes {
    const MAX_CHARS: usize = 500;

    pub fn new(value: impl Into<String>) -> Result<Self, NotesError> {
        let s = value.into().trim().to_owned();
        if s.is_empty()              { return Err(NotesError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(NotesError::TooLong(Self::MAX_CHARS)) }
        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum NotesError {
    #[error("las notas no pueden estar vacías")]
    Empty,
    #[error("las notas no pueden tener más de {0} caracteres")]
    TooLong(usize),
}
