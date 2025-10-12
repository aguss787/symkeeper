use std::collections::HashSet;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to load config ({0}): {1}")]
    LoadConfig(String, anyhow::Error),
    #[error("Source file does not exist:\n{}", format_files(.0))]
    TargetFileNotExist(HashSet<String>),
    #[error("Symlink already exists, use --force to overwrite:\n{}", format_files(.0))]
    SymlinkExists(HashSet<String>),
    #[error("Failed to remove existing file/symlink at {0}: {1}")]
    FileCannotBeRemoved(String, std::io::Error),
    #[error("Failed to create parent directory at {0}: {1}")]
    FailedToCreateParentDir(String, std::io::Error),
    #[error("Failed to create symlink from {link} to {target}: {error}")]
    FailedToCreateSymlink {
        link: String,
        target: String,
        error: std::io::Error,
    },
    #[error("Failed to resolve environment variable expansion: {0}")]
    EnvExpansion(anyhow::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn load_config(file: impl ToString, err: impl Into<anyhow::Error>) -> Self {
        Self::LoadConfig(file.to_string(), err.into())
    }
}

fn format_files<'a>(missing_sources: impl IntoIterator<Item = &'a String>) -> String {
    missing_sources
        .into_iter()
        .map(|s| format!("- {}", s))
        .collect::<Vec<_>>()
        .join("\n")
}
