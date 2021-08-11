use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct PathBufWithFileThatMustExist(PathBuf);

impl PathBufWithFileThatMustExist {
    pub fn value(&self) -> &PathBuf {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

impl From<PathBuf> for PathBufWithFileThatMustExist {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}

impl FromStr for PathBufWithFileThatMustExist {
    type Err = IoError;

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        if Path::new(p).exists() {
            Ok(PathBufWithFileThatMustExist(PathBuf::from(p)))
        } else {
            Err(IoError::new(
                ErrorKind::NotFound,
                format!("the file does not exist: {}", p),
            ))
        }
    }
}

impl AsRef<Path> for PathBufWithFileThatMustExist {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}
