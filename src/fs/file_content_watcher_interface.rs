use super::{file_content_interface::*, path_watcher::FsWatcherModule, prelude::*};
pub struct FileContentWatcherInterface<P: AsRef<Path>> {
    inner: FsWatcherModule<P>,
}

impl<'input, P: AsRef<Path>> interface::InterfaceRead<'input> for FileContentWatcherInterface<P> {
    type Input = <FileContentInterface as interface::InterfaceRead<'input>>::Input;
    type Output = <FileContentInterface as interface::InterfaceRead<'input>>::Output;
    type OutputError = <FileContentInterface as interface::InterfaceRead<'input>>::OutputError;
    fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
        FileContentInterface::read(input)
    }
}
impl<P: AsRef<Path>> FileContentWatcherInterface<P> {
    pub fn get(&self) -> &FsWatcherModule<P> {
        &self.inner
    }
    pub fn iter(&self) -> hash_map::Iter<'_, P, RecursiveMode> {
        self.inner.iter()
    }
}
impl<P: AsRef<Path>> FileContentWatcherInterface<P>
where
    P: Eq + Hash + Clone,
{
    pub fn new(
        paths: &[(P, RecursiveMode)],
    ) -> Result<VecResult<FileContentWatcherInterface<P>, Vec<error::Error>>> {
        Ok(FsWatcherModule::new(paths)?.map(|x, e| (FileContentWatcherInterface { inner: x }, e)))
    }
}
