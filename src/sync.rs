use std::{collections::HashSet, path::Path};

use crate::prelude::*;

/// Sync symlinks
///
/// Logic flow:
/// - Check if all source files exist
/// - Remove all existing file/symlinks
/// - Create symlinks
///     - Make sure parent directories exist
pub(crate) fn run(config: Config) -> Result<()> {
    let symlinks = config
        .symlinks
        .iter()
        .map(|s| s.to_path_buf())
        .collect::<Result<Vec<_>>>()?;

    let missing_targets = symlinks
        .iter()
        .map(Symlink::target)
        .filter_map(|target| (!target.exists()).then(|| target.to_owned_string_lossy()))
        .collect::<HashSet<_>>();
    if !missing_targets.is_empty() {
        return Err(Error::TargetFileNotExist(missing_targets));
    }

    for Symlink { link, .. } in &symlinks {
        if link.exists() {
            println!("Removing existing file/symlink at {}", link.display());
            std::fs::remove_dir_all(link)
                .map_err(|error| Error::FileCannotBeRemoved(link.to_owned_string_lossy(), error))?;
        }
    }

    for Symlink { target, link } in symlinks {
        let link_parent = link.parent();
        if let Some(link_parent) = link_parent {
            println!("Creating parent directory at {}", link_parent.display());
            std::fs::create_dir_all(link_parent).map_err(|error| {
                Error::FailedToCreateParentDir(link_parent.to_owned_string_lossy(), error)
            })?;
        }

        println!(
            "Creating symlink from {} to {}",
            link.display(),
            target.display()
        );
        std::os::unix::fs::symlink(&target, &link).map_err(|error| {
            Error::FailedToCreateSymlink {
                link: link.to_owned_string_lossy(),
                target: target.to_owned_string_lossy(),
                error,
            }
        })?;
    }

    Ok(())
}

trait PathExt {
    fn to_owned_string_lossy(&self) -> String;
}

impl PathExt for Path {
    fn to_owned_string_lossy(&self) -> String {
        self.to_string_lossy().to_string()
    }
}
