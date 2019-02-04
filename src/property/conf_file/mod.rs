//! Set values in configuration files.
use super::super::util::UserPathBuf;

mod internal;

use self::internal::ConfFile;

/// Commonly used syntax
pub fn classic_syntax<P: Into<UserPathBuf>>(file: P) -> ConfFile {
    with_syntax(file, '#', '=')
}

/// Simple assignment syntax using other symbols
pub fn with_syntax<P: Into<UserPathBuf>>(file: P, comment: char, equal: char) -> ConfFile {
    let file = file.into();
    ConfFile {
        path: file,
        comment,
        equal,
    }
}
