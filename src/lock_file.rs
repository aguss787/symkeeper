use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct LockFilePath(PathBuf);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LockFile {
    #[serde(default)]
    pub(crate) symlinks: Symlinks,
    #[serde(default)]
    pub(crate) symlinks_to_remove: BTreeSet<PathBuf>,
}

impl LockFile {
    pub(crate) fn new(symlinks: Symlinks, symlinks_to_remove: BTreeSet<PathBuf>) -> Self {
        Self {
            symlinks,
            symlinks_to_remove,
        }
    }
}

impl LockFilePath {
    pub(crate) fn from_config_path(config_path: impl AsRef<Path>) -> Self {
        let config_path = config_path.as_ref();
        let lock_file_path = config_path.with_extension("lock");
        Self(lock_file_path)
    }

    pub(crate) fn load(&self) -> Result<Option<LockFile>> {
        if !self.0.exists() {
            return Ok(None);
        }

        let lock_file = std::fs::read_to_string(&self.0).map_err(|error| {
            Error::LoadLockFile(self.0.to_string_lossy().to_string(), error.into())
        })?;

        toml::from_str(&lock_file).map(Some).map_err(|error| {
            Error::LoadLockFile(self.0.to_string_lossy().to_string(), error.into())
        })
    }

    pub(crate) fn save(&self, lock_file: &LockFile) -> Result<()> {
        let lock_file = toml::to_string(lock_file).map_err(|error| {
            Error::SaveLockFile(self.0.to_string_lossy().to_string(), error.into())
        })?;

        std::fs::write(&self.0, lock_file).map_err(|error| {
            Error::SaveLockFile(self.0.to_string_lossy().to_string(), error.into())
        })
    }
}
