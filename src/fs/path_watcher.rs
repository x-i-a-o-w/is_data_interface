use super::prelude::*;
pub struct FsWatcherModule<P: AsRef<Path> = PathBuf> {
    paths: HashMap<P, RecursiveMode>,
    watcher: PathWatcher,
}

impl<P: AsRef<Path>> FsWatcherModule<P>
where
    P: Eq + Hash + Clone,
{
    pub fn new(
        paths: &[(P, RecursiveMode)],
    ) -> Result<VecResult<FsWatcherModule<P>, Vec<error::Error>>> {
        Ok(PathWatcher::new(paths)?.map(|x, e| {
            (
                FsWatcherModule {
                    paths: paths.iter().map(|x| (x.0.clone(), x.1)).collect(),
                    watcher: x,
                },
                e,
            )
        }))
    }
    pub fn watch(
        &mut self,
        path: P,
        recursive_mode: RecursiveMode,
    ) -> Result<Option<RecursiveMode>> {
        self.watcher
            .watch(path.as_ref(), recursive_mode)
            .map(|_| self.paths.insert(path, recursive_mode))
    }
    pub fn unwatch(&mut self, path: P) -> Result<Option<RecursiveMode>> {
        self.watcher
            .unwatch(path.as_ref())
            .map(|_| self.paths.remove(&path))
    }
}

impl<P: AsRef<Path>> FsWatcherModule<P> {
    pub fn share_reciver(&self) -> PathWatcherReciver {
        self.watcher.reciver.clone()
    }
    pub fn iter(&self) -> hash_map::Iter<'_, P, RecursiveMode> {
        self.paths.iter()
    }
    pub fn drain(&mut self) -> hash_map::Drain<'_, P, RecursiveMode> {
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
    watcher: RecommendedPathWatcher,
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
        let result = VecResult::from_error_fill(paths.iter(), |x| {
            watcher.watch(x.0.as_ref(), x.1).map_err(|x| x.into())
        });
        Ok(result.map(|_, e| {
            (
                Self {
                    watcher,
                    reciver: rx.into(),
                },
                e,
            )
        }))
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
