use std::collections::BTreeSet;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Failed to load config at {0}: {1}")]
    LoadConfig(String, anyhow::Error),
    #[error("Failed to load lock file, remove \"{0}\" and run `sync` to regenerate it: {1}")]
    LoadLockFile(String, anyhow::Error),
    #[error("Failed to save lock file at {0}: {1}")]
    SaveLockFile(String, anyhow::Error),
    #[error("Source file does not exist:\n{}", format_files(.0))]
    TargetFileNotExist(BTreeSet<String>),
    #[error("Symlink already exists, use --force to overwrite:\n{}", format_files(.0))]
    SymlinkExists(BTreeSet<String>),
    #[error("Failed to inspect symlink at {0}: {1}")]
    FailedToInspectSymlink(String, std::io::Error),
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
