use std::path::Path;

use crate::prelude::*;

pub(crate) struct Fs {
    dry_run: bool,
}

impl Fs {
    pub(crate) fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub(crate) fn remove_dir_all(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if self.dry_run {
            println!("Would remove dir at {}", path.display());
            Ok(())
        } else {
            std::fs::remove_dir_all(path)
                .map_err(|error| Error::FileCannotBeRemoved(path.to_owned_string_lossy(), error))
        }
    }

    pub(crate) fn create_dir_all(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if self.dry_run {
            println!("Would create dir at {}", path.display());
            Ok(())
        } else {
            std::fs::create_dir_all(path).map_err(|error| {
                Error::FailedToCreateParentDir(path.to_owned_string_lossy(), error)
            })
        }
    }

    pub(crate) fn symlink(&self, target: impl AsRef<Path>, link: impl AsRef<Path>) -> Result<()> {
        let link = link.as_ref();
        let target = target.as_ref();
        if self.dry_run {
            println!(
                "Would create symlink from {} to {}",
                link.display(),
                target.display()
            );
            Ok(())
        } else {
            std::os::unix::fs::symlink(target, link).map_err(|error| Error::FailedToCreateSymlink {
                link: link.to_owned_string_lossy(),
                target: target.to_owned_string_lossy(),
                error,
            })
        }
    }
}
