pub mod interface;
pub mod error;
use notify::{RecursiveMode};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, channel},
};

pub type Result<T> = std::result::Result<T,error::Error>;

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
	pub paths:Vec<PathBuf>,
    pub watcher: FileWatcher,
}

impl FsInterface {
    pub fn new<P: AsRef<Path>>(paths:&[(P,RecursiveMode)]) -> Result<FsInterface> {
        Ok(FsInterface {
            paths: paths.iter().map(|x|{
            	PathBuf::from(x.0.as_ref())
            }).collect(),
            watcher: FileWatcher::new(paths)?,
        })
    }
}

pub struct FileWatcher {
    pub watcher: notify::RecommendedWatcher,
    pub reciver: Receiver<notify::Result<notify::Event>>,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(paths: &[(P,RecursiveMode)]) -> Result<Self> {
    	use notify::Watcher;
        let (sx, rx) = channel();
		let mut watcher=notify::recommended_watcher(sx)?;
		let v=paths.iter().filter_map(|x|{watcher.watch(x.0.as_ref(),x.1).err()}).collect::<Vec<_>>();
		if !v.is_empty(){
			return Err(error::Error::Multiple(v));
		}
        Ok(Self {
            watcher,
            reciver: rx,
        })
    }
}
// TODO maybe impl Watcher
// impl notify::Watcher for FileWatcher {
//     fn new<F: notify::EventHandler>(event_handler: F, config: notify::Config) -> Result<Self>
//     where
//         Self: Sized,
//     {
//     	Self::new()
//     }
//     fn watch(&mut self, path: &Path, recursive_mode: notify::RecursiveMode) -> Result<()> {}
//     fn unwatch(&mut self, path: &Path) -> Result<()> {}
//     fn kind() -> notify::WatcherKind
//     where
//         Self: Sized,
//     {
//     }
// }
