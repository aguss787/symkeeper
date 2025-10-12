use std::{collections::BTreeSet, path::Path};

use crate::prelude::*;

pub(crate) struct SyncRunner<'a> {
    fs: &'a Fs,
    lock_file_path: &'a LockFilePath,
    force: bool,
}

impl<'a> SyncRunner<'a> {
    pub(crate) fn new(fs: &'a Fs, lock_file_path: &'a LockFilePath, force: bool) -> Self {
        Self {
            fs,
            lock_file_path,
            force,
        }
    }

    /// Sync symlinks
    ///
    /// Logic flow:
    /// - Check if all source files exist
    /// - If --force is not set, all symlinks must not exist or point to the same target as the lock file
    /// - Create or replace symlinks
    ///     - Make sure parent directories exist
    pub(crate) fn run(self, config: Config) -> Result<()> {
        let missing_targets = config
            .symlinks
            .iter()
            .map(Symlink::into_target)
            .filter(|target| !target.exists())
            .map(|target| target.to_owned_string_lossy())
            .collect::<BTreeSet<_>>();
        if !missing_targets.is_empty() {
            return Err(Error::TargetFileNotExist(missing_targets));
        }

        let lock_file = self.lock_file_path.load()?;
        if !self.force {
            let existing_links = config
                .symlinks
                .iter()
                .map(Symlink::into_link)
                .filter(|link| link.exists())
                .filter_map(|link| match lock_file.require_force(&self.fs, link) {
                    Ok(false) => None,
                    Ok(true) => Some(Ok(link.to_owned_string_lossy())),
                    Err(error) => Some(Err(error)),
                })
                .collect::<Result<BTreeSet<_>>>()?;
            if !existing_links.is_empty() {
                return Err(Error::SymlinkExists(existing_links));
            }
        }

        for Symlink { target, link } in config.symlinks.iter() {
            if link.exists() {
                println!("Removing existing file/symlink at {}", link.display());
                self.fs.remove_dir_all(link)?;
            }

            let link_parent = link.parent();
            if let Some(link_parent) = link_parent {
                println!("Creating parent directory at {}", link_parent.display());
                self.fs.create_dir_all(link_parent)?;
            }

            println!(
                "Creating symlink from {} to {}",
                link.display(),
                target.display()
            );
            self.fs.symlink(target, link)?;
        }

        let (lock_file_symlinks, lock_file_symlinks_to_remove) = lock_file.map_or_else(
            Default::default,
            |LockFile {
                 symlinks,
                 symlinks_to_remove,
             }| (symlinks, symlinks_to_remove),
        );
        let symlinks_to_remove = lock_file_symlinks
            .into_iter()
            .map(Symlink::into_link)
            .chain(lock_file_symlinks_to_remove.into_iter())
            .filter(|link| config.symlinks.get(link).is_none())
            .collect();

        let lock_file = LockFile::new(config.symlinks, symlinks_to_remove);
        self.lock_file_path.save(&lock_file)?;

        Ok(())
    }
}

trait LockFileSymlinkChecker {
    fn require_force(&self, fs: &Fs, link: impl AsRef<Path>) -> Result<bool>;
}

impl LockFileSymlinkChecker for Option<LockFile> {
    fn require_force(&self, fs: &Fs, link: impl AsRef<Path>) -> Result<bool> {
        let link = link.as_ref();
        if !link.exists() {
            return Ok(false);
        }

        let Some(lock_file) = self else {
            return Ok(true);
        };

        if lock_file.symlinks_to_remove.contains(link) {
            return Ok(false);
        }

        let Some(lock_file_target) = lock_file.symlinks.get(link) else {
            return Ok(true);
        };

        let target = fs.symlink_target(link)?;
        Ok(target.as_deref() != Some(lock_file_target))
    }
}
