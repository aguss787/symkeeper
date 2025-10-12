use std::path::Path;

pub(crate) trait PathExt {
    fn to_owned_string_lossy(&self) -> String;
}

impl PathExt for Path {
    fn to_owned_string_lossy(&self) -> String {
        self.to_string_lossy().to_string()
    }
}
