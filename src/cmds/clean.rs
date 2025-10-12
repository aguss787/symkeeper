use crate::prelude::*;

pub(crate) struct CleanRunner<'a> {
    fs: &'a Fs,
    lock_file_path: &'a LockFilePath,
}

impl<'a> CleanRunner<'a> {
    pub(crate) fn new(fs: &'a Fs, lock_file_path: &'a LockFilePath) -> Self {
        Self { fs, lock_file_path }
    }

    pub(crate) fn run(self) -> Result<()> {
        let Some(mut lock_file) = self.lock_file_path.load()? else {
            println!("No lock file found, nothing to clean");
            return Ok(());
        };

        for link in std::mem::take(&mut lock_file.symlinks_to_remove) {
            if link.exists() {
                println!("Removing symlink at {}", link.display());
                self.fs.remove_dir_all(link)?;
            }
        }

        self.lock_file_path.save(&lock_file)?;
        Ok(())
    }
}
