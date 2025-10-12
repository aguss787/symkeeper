use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) symlinks: Symlinks,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Symlinks(HashMap<String, String>);

impl Config {
    pub(crate) fn load(config_file: Option<PathBuf>) -> Result<Self> {
        let config_file = config_file.map_or_else(
            || {
                std::env::current_dir()
                    .map(|current_dir| current_dir.join(".config"))
                    .map_err(Error::load_config)
            },
            Ok,
        )?;

        let toml = std::fs::read_to_string(config_file).map_err(Error::load_config)?;
        toml::from_str(&toml).map_err(Error::load_config)
    }
}

impl Symlinks {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Symlink<&str>> {
        self.0.iter().map(|(link, target)| Symlink {
            target: target.as_str(),
            link: link.as_str(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config_1 = Config::load(Some("fixtures/config_1.toml".into())).unwrap();
        assert_eq!(config_1.symlinks.0.len(), 1);
        assert_eq!(config_1.symlinks.0["foo"], "bar");

        let config_2 = Config::load(Some("fixtures/config_2.toml".into())).unwrap();
        assert_eq!(config_2.symlinks.0.len(), 1);
        assert_eq!(config_2.symlinks.0["foo_2"], "bar_2");
    }

    #[test]
    fn test_duplicate_symlinks() {
        let err = Config::load(Some("fixtures/duplicate_symlinks.toml".into())).unwrap_err();
        assert!(err.to_string().contains("duplicate key"), "{}", err);
    }
}
