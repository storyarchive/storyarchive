use std::path::{Path, PathBuf};

pub mod db;

pub fn join<P, Ps>(parts: Ps) -> PathBuf
    where Ps: IntoIterator<Item=P>,
          P: AsRef<Path>
{
    parts.into_iter().fold(PathBuf::new(), |mut acc, p| {
        acc.push(p);
        acc
    })
}
