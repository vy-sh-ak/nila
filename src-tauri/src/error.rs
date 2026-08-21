use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Sql(String),
    Crypto(String),
    Store(String),
    NotFound(String),
    Validation(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sql(msg) => write!(f, "database error: {msg}"),
            Self::Crypto(msg) => write!(f, "crypto error: {msg}"),
            Self::Store(msg) => write!(f, "store error: {msg}"),
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sql(err.to_string())
    }
}

impl From<aes_gcm::Error> for AppError {
    fn from(err: aes_gcm::Error) -> Self {
        Self::Crypto(err.to_string())
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Crypto(err.to_string())
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
