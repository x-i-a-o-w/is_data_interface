use std::{
    ffi::OsStr,
    fs::rename,
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub struct RenameFileLock<'lock, State = Unlocked> {
    paths: Vec<&'lock Path>,
    // unlock_on_drop:bool,
    state: PhantomData<State>,
}

pub struct Unlocked;
pub struct Locked;

trait State {}

impl State for Unlocked {}
impl State for Locked {}
// TODO modify lock so that it detects if a path is a file so it doesn't rename it or renames it into something else

impl<'lock> RenameFileLock<'lock, Unlocked> {
    pub fn new(paths: Vec<&'lock Path>) -> RenameFileLock<'lock, Unlocked> {
        RenameFileLock {
            paths,
            // unlock_on_drop:false,
            state: PhantomData::<Unlocked>,
        }
    }
    pub fn lock<P: AsRef<Path>>(self, path: P) -> std::io::Result<RenameFileLock<'lock, Locked>> {
        self.lock_with_extension(path.as_ref(), Path::new(LOCK_EXTENSION))
    }
    pub fn lock_with_extension<P: AsRef<Path>, S: AsRef<OsStr>>(
        self,
        path: P,
        extension: S,
    ) -> std::io::Result<RenameFileLock<'lock, Locked>> {
        rename(
            path.as_ref(),
            path.as_ref().with_added_extension(extension.as_ref()),
        )
        .map(|_| RenameFileLock {
            paths: self.paths,
            state: std::marker::PhantomData::<Locked>,
        })
    }
    pub fn iter(&self) -> Iter<'_, &str> {
        Iter {
            iter: self.paths.iter(),
            locked: false,
            extension: LOCK_EXTENSION,
        }
    }
    pub fn iter_extension<S: AsRef<OsStr>>(&self, extension: S) -> Iter<'_, S> {
        Iter {
            iter: self.paths.iter(),
            locked: false,
            extension,
        }
    }
}

impl<'lock> RenameFileLock<'lock, Locked> {
    pub fn unlock<P: AsRef<Path>>(
        self,
        path: P,
    ) -> std::io::Result<RenameFileLock<'lock, Unlocked>> {
        self.unlock_with_extension(path.as_ref(), Path::new(LOCK_EXTENSION))
    }
    pub fn unlock_with_extension<P: AsRef<Path>, S: AsRef<OsStr>>(
        self,
        path: P,
        extension: S,
    ) -> std::io::Result<RenameFileLock<'lock, Unlocked>> {
        rename(path.as_ref().with_added_extension(extension.as_ref()), path).map(|_| {
            RenameFileLock {
                paths: self.paths,
                state: std::marker::PhantomData::<Unlocked>,
            }
        })
    }
    pub fn iter(&self) -> Iter<'_, &str> {
        Iter {
            iter: self.paths.iter(),
            locked: false,
            extension: LOCK_EXTENSION,
        }
    }
    pub fn iter_extension<S: AsRef<OsStr>>(&self, extension: S) -> Iter<'_, S> {
        Iter {
            iter: self.paths.iter(),
            locked: false,
            extension,
        }
    }
}

pub struct Iter<'lock, S: AsRef<OsStr>> {
    iter: std::slice::Iter<'lock, &'lock Path>,
    locked: bool,
    extension: S,
}

impl<'lock, S> Iterator for Iter<'lock, S>
where
    S: AsRef<OsStr>,
{
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| {
            if self.locked {
                x.with_added_extension(self.extension.as_ref())
            } else {
                x.to_path_buf()
            }
        })
    }
}

pub const LOCK_EXTENSION: &str = "islock";
