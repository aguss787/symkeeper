use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Symlinks<T: Ord = PathBuf>(BTreeMap<T, T>);

impl Symlinks<String> {
    pub(crate) fn to_path_bufs(&self) -> Result<Symlinks> {
        self.0
            .iter()
            .map(|(link, target)| {
                Symlink {
                    target: target.as_str(),
                    link: link.as_str(),
                }
                .to_path_buf()
            })
            .collect::<Result<_>>()
    }
}

impl FromIterator<Symlink<PathBuf>> for Symlinks {
    fn from_iter<I: IntoIterator<Item = Symlink<PathBuf>>>(iter: I) -> Self {
        Self(BTreeMap::from_iter(
            iter.into_iter()
                .map(|Symlink { target, link }| (link, target)),
        ))
    }
}

impl Symlinks {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Symlink<&Path>> {
        self.0.iter().map(|(link, target)| Symlink {
            target: target.as_path(),
            link: link.as_path(),
        })
    }

    pub(crate) fn get(&self, link: &Path) -> Option<&Path> {
        self.0.get(link).map(|target| target.as_path())
    }

    #[cfg(test)]
    pub(crate) fn into_inner(self) -> BTreeMap<PathBuf, PathBuf> {
        self.0
    }
}

impl<T: Ord> Symlinks<T> {
    pub(crate) fn into_iter(self) -> impl Iterator<Item = Symlink<T>> {
        self.0
            .into_iter()
            .map(|(link, target)| Symlink { target, link })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Symlink<T> {
    pub(crate) target: T,
    pub(crate) link: T,
}

impl<T> Symlink<T> {
    pub(crate) fn into_target(self) -> T {
        self.target
    }

    pub(crate) fn into_link(self) -> T {
        self.link
    }

    fn try_map<U, E>(
        self,
        f: impl Fn(T) -> std::result::Result<U, E>,
    ) -> std::result::Result<Symlink<U>, E> {
        Ok(Symlink {
            target: f(self.target)?,
            link: f(self.link)?,
        })
    }
}

impl Symlink<&str> {
    pub(crate) fn to_path_buf(self) -> Result<Symlink<PathBuf>> {
        self.try_map(|s| Ok(Path::new(&expand_env_variable(s)?).to_path_buf()))
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
