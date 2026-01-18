use std::path::Path;

pub trait PathExt {
    fn normalize(&self) -> String;
}

impl PathExt for Path {
    fn normalize(&self) -> String {
        let s = self.display().to_string();

        #[cfg(windows)]
        {
            // Remove Windows extended-length path prefix
            if let Some(stripped) = s.strip_prefix(r"\\?\") {
                return stripped.to_string();
            }
        }

        s
    }
}
