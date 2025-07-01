use crate::dfs::{Directory, FileName, FileSpec};
use anyhow::Error;
use std::str::FromStr;

pub struct DfsPath {
    pub directory: Directory,
    pub file_name: FileName,
}

impl FromStr for DfsPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directory, s) = match s.split_once('.') {
            Some((prefix, suffix)) => {
                if prefix.len() == 1 {
                    let c = prefix
                        .chars()
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap()
                        .to_owned();
                    (c.try_into().unwrap_or(Directory::ROOT), suffix)
                } else {
                    (Directory::ROOT, s)
                }
            }
            _ => (Directory::ROOT, s),
        };

        let file_name = s.parse()?;
        Ok(Self {
            directory,
            file_name,
        })
    }
}

impl FileSpec for DfsPath {
    fn directory(&self) -> &Directory {
        &self.directory
    }

    fn file_name(&self) -> &FileName {
        &self.file_name
    }
}
