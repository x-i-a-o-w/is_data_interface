use super::prelude::*;
pub type FileEntry<P:AsRef<Path>> = (P, Vec<u8>);
pub type FileEntryError = (PathBuf, error::Error);
pub type FileEntryResult = (PathBuf, Result<Vec<u8>>);

pub struct FileContentInterface<P: AsRef<Path> = PathBuf> {
    paths: HashSet<P>,
}
// new -> Self functions should not return Result<T>
impl<P> FileContentInterface<P>
where
    P: AsRef<Path> + Eq + Hash,
{
    pub fn new<S: Into<HashSet<P>>>(s: S) -> FileContentInterface<P> {
        FileContentInterface { paths: s.into() }
    }
    pub fn iter(&self) -> hash_set::Iter<'_, P> {
        self.paths.iter()
    }
}

impl<'input,P:AsRef<Path>> interface::InterfaceRead<'input> for FileContentInterface<P>
	where P:'input
 {
    type Input = &'input P;
    type Output = Vec<u8>;
    type OutputError = error::Error;
    fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
        Ok(fs::read(input)?)
    }
}

impl<'input,Powned,P:AsRef<Path>> interface::InterfaceReadManager<'input> for FileContentInterface<P>
	where 
		// FileContentInterface<P>: interface::InterfaceRead<'input> ,
		P:'input+ToOwned<Owned=Powned>,
		
 {
    type OutputCollection<O: 'input> = VecResult<Vec<(Powned,O)>, Vec<(Powned,error::Error)>>;
    fn read_all(
        &'input self,
    ) -> anyhow::Result<Self::OutputCollection<Self::Output>> {
        use interface::InterfaceRead;
        let mut v: Self::OutputCollection<Self::Output> = VecResult{ok:Vec::new(),err:Vec::new()};
        for i in &self.paths {
            VecResult::push_result(
                v.as_mut(),
                Self::read(i)
                    .map(|x| (i.to_owned(), x))
                    .map_err(|x| (i.to_owned(), x))
            );
        }
        Ok(v)
    }
}

impl<P: AsRef<Path>> AsMut<HashSet<P>> for FileContentInterface<P> {
    fn as_mut(&mut self) -> &mut HashSet<P> {
        &mut self.paths
    }
}
impl<P: AsRef<Path>> AsRef<HashSet<P>> for FileContentInterface<P> {
    fn as_ref(&self) -> &HashSet<P> {
        &self.paths
    }
}

impl<P> FromIterator<P> for FileContentInterface<P>
where
    P: AsRef<Path> + Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        FileContentInterface {
            paths: HashSet::from_iter(iter),
        }
    }
}
