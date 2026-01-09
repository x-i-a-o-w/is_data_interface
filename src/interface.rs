/// InterfaceRead that can read from a source and output the source as Result
pub trait InterfaceRead<'input> {
    type Input;
    type Output;
    type OutputError;
    fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError>;
}

pub trait InterfaceReadManager<'input>: InterfaceRead<'input> {
    type OutputCollection<O>
    where
        Self: 'input,
        O: 'input;
    fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>>;
}

/// InterfaceRead has the ability to fix the InterfaceRead if read returns Err(e)
pub trait RecoverInterfaceRead<'input>: InterfaceRead<'input> {
    /// recover may fail if InterfaceRead cannot be made with write
    /// this function should only fail because of stuff not existing or not having perms or something that he user cannot modify thru program himself
    fn recover(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError>;
}

pub trait InterfaceReadIterManager<'input>: InterfaceRead<'input> {
    type OutputCollection<O: 'input>
    where
        Self: 'input;
    type Pool: Iterator;
    fn pool(&'input self) -> Self::Pool;
    fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>>
    where
        Self::Input: From<<Self::Pool as Iterator>::Item>,
        Self::OutputCollection<Self::Output>:
            FromIterator<std::result::Result<Self::Output, Self::OutputError>>,
    {
        Ok(self.pool().map(|x| Self::read(x.into())).collect())
    }
}
