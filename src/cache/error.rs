use std::fmt;

#[derive(Debug)]
pub enum CacheError {
    Io(std::io::Error),
    Json(serde_json::Error),
    HomeNotFound,
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Io(err) => write!(f, "IO error: {}", err),
            CacheError::Json(err) => write!(f, "JSON serialization error: {}", err),
            CacheError::HomeNotFound => write!(f, "Home directory not found"),
        }
    }
}

impl std::error::Error for CacheError {}

impl From<std::io::Error> for CacheError {
    fn from(err: std::io::Error) -> Self {
        CacheError::Io(err)
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::Json(err)
    }
}