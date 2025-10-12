use std::collections::HashMap;

use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) symlinks: Symlinks,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Symlinks(HashMap<String, String>);

impl Config {
    pub(crate) fn load() -> Result<Self> {
        let config_path = std::env::current_dir()
            .map_err(Error::load_config)?
            .join("symkeeper.toml");
        let toml = std::fs::read_to_string(config_path).map_err(Error::load_config)?;
        Self::load_from_str(&toml)
    }

    fn load_from_str(toml: &str) -> Result<Self> {
        toml::from_str(&toml).map_err(Error::load_config)
    }
}

impl Symlinks {
    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = Symlink<&'a str>> {
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
    fn test_duplicate_symlinks() {
        let raw = r#"
            [symlinks]
            "foo" = "bar"
            "foo" = "baz"
        "#;
        let err = Config::load_from_str(raw).unwrap_err();
        assert!(err.to_string().contains("duplicate key"), "{}", err);
    }
}
