#[derive(Debug, Clone)]
pub struct Phone(String);

impl Phone {
    pub fn new(raw: String) -> Result<Self, PhoneError> {
        let normalized: String = raw
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();

        if !normalized.starts_with('+') {
            return Err(PhoneError::Invalid);
        }

        let digits = &normalized[1..];
        if !digits.chars().all(|c| c.is_ascii_digit()) || !(8..=15).contains(&digits.len()) {
            return Err(PhoneError::Invalid);
        }

        Ok(Self(normalized))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    #[error("teléfono inválido: debe tener formato E.164 (ej. +5491112345678), entre 8 y 15 dígitos después del '+'")]
    Invalid,
}
