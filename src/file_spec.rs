use crate::directory::Directory;
use crate::file_name::FileName;
use std::cmp::Ordering;

pub trait FileSpec {
    fn directory(&self) -> &Directory;

    fn file_name(&self) -> &FileName;

    fn compare(a: &Self, b: &Self) -> Ordering {
        match a.directory().partial_cmp(b.directory()) {
            Some(ordering) if ordering != Ordering::Equal => return ordering,
            _ => {}
        }
        match a.file_name().partial_cmp(b.file_name()) {
            Some(ordering) if ordering != Ordering::Equal => return ordering,
            _ => {}
        }
        Ordering::Equal
    }
}
