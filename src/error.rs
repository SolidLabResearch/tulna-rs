use thiserror::Error;

#[derive(Error, Debug)]
pub enum TulnaError {
    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Unsupported query language feature: {0}")]
    UnsupportedFeature(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}
