use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineErrorKind {
    Authorization,
    Transcription,
    Generation,
    Validation,
    Telemetry,
    IterationLimit,
    InvalidInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineError {
    pub kind: EngineErrorKind,
    pub code: &'static str,
    pub message: String,
}

impl EngineError {
    pub fn new(kind: EngineErrorKind, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
        }
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for EngineError {}

pub type EngineResult<T> = Result<T, EngineError>;
