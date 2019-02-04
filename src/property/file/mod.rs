use self::internal::ManagedFile;
use crate::property::file::internal::ContainsLines;
use crate::property::file::internal::ContentBytes;
use crate::util::UserPathBuf;
use std::sync::Arc;

mod internal;

pub fn file<P: Into<UserPathBuf>>(path: P) -> ManagedFile {
    let path = path.into();
    let path = Arc::new(path);
    ManagedFile { path }
}

impl ManagedFile {
    /// Content of the file is exactly the same as the provided bytes
    pub fn content_bytes(&self, bytes: &'static [u8]) -> ContentBytes {
        ContentBytes {
            path: self.path.clone(),
            bytes,
        }
    }

    #[allow(dead_code)]
    pub fn contains_line<S>(&self, line: S) -> ContainsLines
    where
        S: AsRef<str>,
    {
        self.contains_lines(&[line])
    }

    pub fn contains_lines<S>(&self, lines: &[S]) -> ContainsLines
    where
        S: AsRef<str>,
    {
        let file = self.path.clone();
        let lines = lines.iter().map(|l| l.as_ref().to_string()).collect();
        ContainsLines {
            file,
            lines: Arc::new(lines),
        }
    }
}
