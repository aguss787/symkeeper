use std::collections::HashSet;

use crate::prelude::*;

pub(crate) struct SyncRunner {
    fs: Fs,
    force: bool,
}

impl SyncRunner {
    pub(crate) fn new(fs: Fs, force: bool) -> Self {
        Self { fs, force }
    }

    /// Sync symlinks
    ///
    /// Logic flow:
    /// - Check if all source files exist
    /// - If --force is not set, all symlinks must not exist
    /// - Create or replace symlinks
    ///     - Make sure parent directories exist
    pub(crate) fn run(self, config: Config) -> Result<()> {
        let symlinks = config
            .symlinks
            .iter()
            .map(|s| s.to_path_buf())
            .collect::<Result<Vec<_>>>()?;

        let missing_targets = symlinks
            .iter()
            .map(Symlink::target)
            .filter(|target| !target.exists())
            .map(|target| target.to_owned_string_lossy())
            .collect::<HashSet<_>>();
        if !missing_targets.is_empty() {
            return Err(Error::TargetFileNotExist(missing_targets));
        }

        if !self.force {
            let existing_links = symlinks
                .iter()
                .map(Symlink::link)
                .filter(|link| link.exists())
                .map(|link| link.to_owned_string_lossy())
                .collect::<HashSet<_>>();
            if !existing_links.is_empty() {
                return Err(Error::SymlinkExists(existing_links));
            }
        }

        for Symlink { target, link } in symlinks {
            if link.exists() {
                println!("Removing existing file/symlink at {}", link.display());
                self.fs.remove_dir_all(&link)?;
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
            self.fs.symlink(&target, &link)?;
        }

        Ok(())
    }
}
