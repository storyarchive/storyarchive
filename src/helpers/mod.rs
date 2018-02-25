use std::path::{Path, PathBuf};

pub mod db;
pub mod fluent;

pub fn join<P, Ps>(parts: Ps) -> PathBuf
    where Ps: IntoIterator<Item=P>,
          P: AsRef<Path>
{
    parts.into_iter().fold(PathBuf::new(), |mut acc, p| {
        acc.push(p);
        acc
    })
}
