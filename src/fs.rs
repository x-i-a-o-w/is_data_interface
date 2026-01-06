use crate::{
    Result,
    error::{self, VecResult},
    interface,
};
use notify::{RecursiveMode, Watcher};
use std::{
    collections::{HashMap, hash_map},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc::{Receiver, channel},
};

pub type FileEntry = (PathBuf, Vec<u8>);
pub type FileEntryError = (PathBuf, error::Error);
pub type FileEntryResult = (PathBuf, Result<Vec<u8>>);

/// this is used as a F generic for its container structs
/// can be replaced with another data structure(this struct is for quick use)
/// used as a predefined struct to read_ignore
pub struct FsInterface {
    paths: HashMap<PathBuf, RecursiveMode>,
    pub watcher: PathWatcher,
}

impl FsInterface {
    pub fn new<P: AsRef<Path>>(
        paths: &[(P, RecursiveMode)],
    ) -> Result<VecResult<Self, Vec<error::Error>>> {
        Ok(PathWatcher::new(paths)?.map(|x, e| {
            (
                FsInterface {
                    paths: paths
                        .iter()
                        .map(|x| (PathBuf::from(x.0.as_ref()), x.1))
                        .collect(),
                    watcher: x,
                },
                e,
            )
        }))
    }
    pub fn share_reciver(&self) -> PathWatcherReciver {
        self.watcher.reciver.clone()
    }
    pub fn paths(&self) -> hash_map::Iter<'_, PathBuf, RecursiveMode> {
        self.paths.iter()
    }
    // TODO look at tests/interface.rs FIXTHIS
    pub fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<Option<RecursiveMode>> {
        self.watcher
            .watch(path, recursive_mode)
            .map(|_| self.paths.insert(path.to_path_buf(), recursive_mode))
    }
    pub fn unwatch(&mut self, path: &Path) -> Result<Option<RecursiveMode>> {
        self.watcher.unwatch(path).map(|_| self.paths.remove(path))
    }
    pub fn drain(&mut self) -> hash_map::Drain<'_, PathBuf, RecursiveMode> {
        self.paths.drain()
    }
    pub fn kind() -> notify::WatcherKind
    where
        Self: Sized,
    {
        PathWatcher::kind()
    }
}

pub type PathWatcherReciver = Rc<Receiver<notify::Result<notify::Event>>>;

type RecommendedPathWatcher = notify::RecommendedWatcher;

pub struct PathWatcher {
    pub watcher: RecommendedPathWatcher,
    reciver: PathWatcherReciver,
}

impl PathWatcher {
    pub fn new<P: AsRef<Path>>(
        paths: &[(P, RecursiveMode)],
    ) -> Result<VecResult<Self, Vec<error::Error>>> {
        Self::new_config(paths, notify::Config::default())
    }
    pub fn new_config<P: AsRef<Path>>(
        paths: &[(P, RecursiveMode)],
        config: notify::Config,
    ) -> Result<VecResult<Self, Vec<error::Error>>> {
        use notify::Watcher;
        let (sx, rx) = channel();
        let mut watcher = notify::RecommendedWatcher::new(sx, config)?;
        let err = paths
            .iter()
            .filter_map(|x| watcher.watch(x.0.as_ref(), x.1).err().map(|x| x.into()))
            .collect::<Vec<_>>();
        Ok(VecResult {
            ok: Self {
                watcher,
                reciver: rx.into(),
            },
            err,
        })
    }
    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        Ok(RecommendedPathWatcher::watch(
            &mut self.watcher,
            path,
            recursive_mode,
        )?)
    }
    fn unwatch(&mut self, path: &Path) -> Result<()> {
        Ok(RecommendedPathWatcher::unwatch(&mut self.watcher, path)?)
    }
    fn kind() -> notify::WatcherKind
    where
        Self: Sized,
    {
        RecommendedPathWatcher::kind()
    }
}

// impl TryFrom<hash_map::Drain<'_, PathBuf, RecursiveMode>> for FileInterface {
//     type Error = error::Error;
//     // TODO myb turn paths into a hashmap because to make a new watcher you need a (PathBuf,RecursiveMode)
//     fn try_from(
//         map: hash_map::Drain<'_, PathBuf, RecursiveMode>,
//     ) -> std::result::Result<Self, Self::Error> {
//         let d = map.collect::<Vec<_>>();
//         let watcher = PathWatcher::new(&d)?;
//         let paths = HashMap::from_iter(d);
//         Ok(Self(FsInterface { paths, watcher }))
//     }
// }

pub struct FileInterface(FsInterface);
impl interface::Interface for FileInterface {
    type Output = VecResult<Vec<FileEntry>, Vec<FileEntryError>>;
    fn read(&self) -> anyhow::Result<Self::Output> {
        let mut read: Self::Output = VecResult::default();
        self.0.paths().for_each(|x| match fs::read(x.0) {
            Ok(e) => read.ok.push((x.0.clone(), e)),
            Err(e) => read.err.push((x.0.clone(), e.into())),
        });
        Ok(read)
    }
}
impl FileInterface {
    pub fn new(f: FsInterface) -> FileInterface {
        FileInterface(f)
    }
    /// in order to ignore the notification you need to
    /// unwatch the path -> rename path(to stop outer conflict) -> read -> rename path -> watch
    /// this process may fail if between unwatching and renaming someone alters/modifies the path
    /// also you must ensure that before this process starts to happen the file is not being written to, because this may cause file corruption
    pub fn read_ignore(
        &mut self,
    ) -> anyhow::Result<<FileInterface as interface::Interface>::Output> {
        use crate::lock::RenameFileLock;
        // RenameFileLock::lock()
        // let mut read: VecResult<Vec<FileEntry>> = VecResult::default();
        // TODO FIXTHIS
        let map: Vec<_> = self.0.drain().collect();
        let v = VecResult::from_split_fill(map.iter(), |(x, _)| -> Result<Vec<u8>> {
            self.0.unwatch(&x)?;
            Ok(fs::read(&x)?)
        });

        // self.0=
        // let locked = self.0.paths().map(|x| {
        //     self.0.unwatch(x);
        //     let lock = RenameFileLock::new().lock(x);
        // });
        interface::Interface::read(self)
    }
    pub fn iter(&self) -> hash_map::Iter<'_, PathBuf, RecursiveMode> {
        self.0.paths.iter()
    }
}
