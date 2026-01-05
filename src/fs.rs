use crate::{Result, error};
use notify::RecursiveMode;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc::{Receiver, channel},
};

/// this is used as a F generic for its container structs
/// can be replaced with another data structure(this struct is for quick use)
/// used as a predefined struct to read_ignore
pub struct FsInterface {
    paths: Vec<PathBuf>,
    pub watcher: FileWatcher,
}

// TODO make a lock system that renames the file to file-name.isl(is-locked) to ignore/stop fileaccess(write,read,etc) for temporary in order to

impl FsInterface {
    pub fn new<P: AsRef<Path>>(paths: &[(P, RecursiveMode)]) -> Result<FsInterface> {
        Ok(FsInterface {
            paths: paths.iter().map(|x| PathBuf::from(x.0.as_ref())).collect(),
            watcher: FileWatcher::new(paths)?,
        })
    }
    pub fn share_reciver(&self) -> FileWatcherReciver {
        self.watcher.reciver.clone()
    }
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
    // pub fn read_ignore<T:interface::Interface>()->T::Output{}
}

pub type FileWatcherReciver = Rc<Receiver<notify::Result<notify::Event>>>;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    reciver: FileWatcherReciver,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(paths: &[(P, RecursiveMode)]) -> Result<Self> {
        use notify::Watcher;
        let (sx, rx) = channel();
        let mut watcher = notify::recommended_watcher(sx)?;
        let v = paths
            .iter()
            .filter_map(|x| watcher.watch(x.0.as_ref(), x.1).err())
            .collect::<Vec<_>>();
        if !v.is_empty() {
            return Err(error::Error::Multiple(v));
        }
        Ok(Self {
            watcher,
            reciver: rx.into(),
        })
    }
}
