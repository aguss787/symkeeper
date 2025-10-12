use std::path::PathBuf;

use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub(crate) struct Config {
    pub(crate) symlinks: Symlinks,
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    symlinks: Symlinks<String>,
}

impl TryFrom<RawConfig> for Config {
    type Error = Error;

    fn try_from(raw_config: RawConfig) -> Result<Self> {
        Ok(Self {
            symlinks: raw_config.symlinks.to_path_bufs()?,
        })
    }
}

impl Config {
    pub(crate) fn load(config_file: Option<PathBuf>) -> Result<(Self, LockFilePath)> {
        let config_file = config_file.map_or_else(
            || {
                std::env::current_dir()
                    .map(|current_dir| current_dir.join("symkeeper.toml"))
                    .map_err(|error| Error::load_config("symkeeper.toml", error))
            },
            Ok,
        )?;

        let toml = std::fs::read_to_string(&config_file)
            .map_err(|error| Error::load_config(config_file.to_string_lossy(), error))?;
        let config = toml::from_str(&toml)
            .map_err(|error| Error::load_config(config_file.to_string_lossy(), error))?;
        let lock_file_path = LockFilePath::from_config_path(&config_file);
        Ok((config, lock_file_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let (config_1, _) = Config::load(Some("fixtures/config_1.toml".into())).unwrap();
        let symlinks = config_1.symlinks.into_inner();
        assert_eq!(symlinks.len(), 1);
        assert_eq!(symlinks[&PathBuf::from("foo")], PathBuf::from("bar"));

        let (config_2, _) = Config::load(Some("fixtures/config_2.toml".into())).unwrap();
        let symlinks = config_2.symlinks.into_inner();
        assert_eq!(symlinks.len(), 1);
        assert_eq!(symlinks[&PathBuf::from("foo_2")], PathBuf::from("bar_2"));
    }

    #[test]
    fn test_duplicate_symlinks() {
        let err = Config::load(Some("fixtures/duplicate_symlinks.toml".into())).unwrap_err();
        assert!(err.to_string().contains("duplicate key"), "{}", err);
    }
}
