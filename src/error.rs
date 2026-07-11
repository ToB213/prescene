use std::path::PathBuf;
use thiserror::Error;

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
}
