# Symkeeper

Symlink management tool.

## Installation

```bash
cargo install symkeeper --locked
```

## TODO

- [ ] Backup command
- [ ] Write a lock file to record managed symlinks
- [ ] Remove symlinks from filesystem if they are in the lock file, but not in the config
- [ ] Try to restore symlinks from lock file if the current sync fails
