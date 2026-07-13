use std::path::PathBuf;
use thiserror::Error;

// Application-level errors with enough context to explain which operation
// failed and which underlying error caused it.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to read file `{path}`: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse YAML: {0}")]
    ParseYaml(#[from] serde_yaml::Error),

    #[error("failed to write file `{path}`: {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    // Covers malformed Markdown structure, such as missing front matter
    // delimiters, rather than YAML deserialization failures.
    #[error("invalid Markdown input: {0}")]
    InvalidMarkdown(String),
}
