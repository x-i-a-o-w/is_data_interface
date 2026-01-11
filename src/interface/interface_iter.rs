use super::*;
pub trait InterfaceReadIterManager<'input, P: Iterator>: InterfaceRead<'input> {
    type OutputCollection<O: 'input>
    where
        Self: 'input;
    fn pool(&'input self) -> P;
    fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>>
    where
        Self::Input: From<<P as Iterator>::Item>,
        Self::OutputCollection<Self::Output>:
            FromIterator<std::result::Result<Self::Output, Self::OutputError>>,
    {
        Ok(self.pool().map(|x| Self::read(x.into())).collect())
    }
}
