pub mod interface;

use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, channel},
};
type Watcher = notify::RecommendedWatcher;

pub struct Init;
pub struct Open;
#[derive(Clone)]
pub struct DataInterface<I: interface::RigidInterface, State = Init> {
    interface: I,
    state: std::marker::PhantomData<State>,
}

impl<I: interface::RigidInterface> DataInterface<I, Init> {
    pub fn new(interface: I) -> DataInterface<I> {
        DataInterface {
            interface,
            state: std::marker::PhantomData::<Init>,
        }
    }
}

/// this is used as a F generic for its container structs
/// can be replaced with another data structure(this struct is for quick use)
pub struct FsInterface {
    pub path: PathBuf,
    watcher: Option<FileWatcher>,
}

impl FsInterface {
    pub fn new<P: AsRef<Path>>(path: P, watcher: bool) -> notify::Result<FsInterface> {
        Ok(FsInterface {
            path: PathBuf::from(path.as_ref()),
            watcher:watcher.then_some(FileWatcher::new(path.as_ref())?),
        })
    }
}
pub struct FileWatcher {
    watcher: Option<Watcher>,
    reciver: Receiver<notify::Result<notify::Event>>,
}
impl FileWatcher {
    pub fn new<P: AsRef<Path>>(_path: P) -> notify::Result<Self> {
        let (sx, rx) = channel();
        Ok(Self {
            watcher: Some(notify::recommended_watcher(sx)?),
            reciver: rx,
        })
    }
}
