use derive_more::Display;
use endpoint_libs::libs::error_code::ErrorCode;
use std::fmt::Display;

pub type HoneyIdResult<T> = Result<T, HoneyIdError>;

#[derive(Debug, Display, Clone)]
#[display("{msg}")]
pub struct HoneyIdError {
    pub code: ErrorCode,
    pub msg: String,
}

impl HoneyIdError {
    pub fn new(code: impl Into<ErrorCode>, msg: impl Display) -> Self {
        Self {
            code: code.into(),
            msg: msg.to_string(),
        }
    }
}

impl From<eyre::Report> for HoneyIdError {
    fn from(value: eyre::Report) -> Self {
        value
            .downcast::<Self>()
            .unwrap_or_else(|e| Self::new(ErrorCode::INTERNAL_ERROR, e))
    }
}
