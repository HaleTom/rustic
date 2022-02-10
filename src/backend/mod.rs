use crate::id::*;
use anyhow::anyhow;

pub mod decrypt;
pub mod local;

pub use decrypt::DecryptBackend;
pub use local::LocalBackend;

#[derive(Clone, Copy)]
pub enum FileType {
    Config,
    Index,
    Key,
    Snapshot,
    Pack,
}

impl FileType {
    pub fn name(&self) -> &str {
        match &self {
            FileType::Config => "config",
            FileType::Snapshot => "snapshots",
            FileType::Index => "index",
            FileType::Key => "keys",
            FileType::Pack => "data",
        }
    }
}

pub trait ReadBackend: Clone {
    type Error: Send + Sync + std::error::Error + 'static;

    fn location(&self) -> &str;
    fn list(&self, tpe: FileType) -> Result<Vec<Id>, Self::Error>;
    fn list_with_size(&self, tpe: FileType) -> Result<Vec<(Id, u32)>, Self::Error>;
    fn read_full(&self, tpe: FileType, id: Id) -> Result<Vec<u8>, Self::Error>;
    fn read_partial(
        &self,
        tpe: FileType,
        id: Id,
        offset: u32,
        length: u32,
    ) -> Result<Vec<u8>, Self::Error>;

    fn find_starts_with(
        &self,
        tpe: FileType,
        vec: &[&str],
    ) -> Result<Vec<Result<Id, anyhow::Error>>, Self::Error> {
        self.map_list(tpe, vec, |s, id| id.to_hex().starts_with(s))
            .map(|res| {
                res.into_iter()
                    .enumerate()
                    .map(|(i, id)| match id {
                        MapResult::Some(id) => Ok(id),
                        MapResult::None => Err(anyhow!("no suitable id found for {}", &vec[i])),
                        MapResult::NonUnique => Err(anyhow!("id {} is not unique", &vec[i])),
                    })
                    .collect()
            })
    }

    /// map_list
    fn map_list<T>(
        &self,
        tpe: FileType,
        vec: &[T],
        matches: impl Fn(&T, Id) -> bool,
    ) -> Result<Vec<MapResult<Id>>, Self::Error> {
        let mut res = vec![MapResult::None; vec.len()];
        for id in self.list(tpe)? {
            for (i, v) in vec.iter().enumerate() {
                if matches(v, id) {
                    if res[i] == MapResult::None {
                        res[i] = MapResult::Some(id);
                    } else {
                        res[i] = MapResult::NonUnique;
                    }
                }
            }
        }

        Ok(res)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MapResult<T> {
    None,
    Some(T),
    NonUnique,
}

/*
pub trait ReadSource: Clone {
    fn walker(&self) -> &dyn Iterator<Item: PathBuf>;

    fn metadata(&self, item: PathBuf) -> MetaData;

    fn read(&self, item: PathBuf) -> &dyn io::Read;

    fn read_partial(
        &self,
        item: PathBuf,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, Self::Error>;

}

pub trait WriteSource: Clone {
    fn create(&self, item: PathBuf);

    fn set_metadata(&self, item: PathBuf, metadata: MetaData);

    fn write_at(&self, item: PathBuf, offset: u64, Vec<u8>);
}
*/
