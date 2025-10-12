use std::path::{Path, PathBuf};

use crate::prelude::*;

pub(crate) struct Symlink<T> {
    pub(crate) target: T,
    pub(crate) link: T,
}

impl<T> Symlink<T> {
    pub(crate) fn target(&self) -> &T {
        &self.target
    }
}

impl Symlink<&str> {
    pub(crate) fn to_path_buf(&self) -> Result<Symlink<PathBuf>> {
        let target = expand_env_variable(self.target)?;
        let link = expand_env_variable(self.link)?;
        Ok(Symlink {
            target: Path::new(&target).to_path_buf(),
            link: Path::new(&link).to_path_buf(),
        })
    }
}

fn expand_env_variable(s: &str) -> Result<String> {
    shellexpand::full(s)
        .map(|s| s.to_string())
        .map_err(|e| Error::EnvExpansion(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_path_expand_env_variable() {
        let symlink = Symlink {
            target: "$HOME/foo",
            link: "bar",
        }
        .to_path_buf()
        .unwrap();
        let home = std::env::var("HOME").unwrap();
        let expected_target = format!("{}/foo", home);

        assert_eq!(symlink.target.to_string_lossy(), expected_target);
        assert_eq!(symlink.link.to_string_lossy(), "bar");
    }
}
