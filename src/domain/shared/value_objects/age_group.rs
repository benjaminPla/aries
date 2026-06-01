#[derive(Debug, Clone, PartialEq, Default)]
pub enum AgeGroup {
    #[default]
    Adult,
    Minor,
}

impl AgeGroup {
    pub fn as_db_str(&self) -> &str {
        match self {
            Self::Adult => "adult",
            Self::Minor => "minor",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "adult" => Some(Self::Adult),
            "minor" => Some(Self::Minor),
            _       => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Adult => "Adulto",
            Self::Minor => "Menor",
        }
    }
}
