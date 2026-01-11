use super::*;
pub trait InterfaceReadManager<'input>: InterfaceRead<'input> {
    type OutputCollection<O>
    where
        Self: 'input,
        O: 'input;
    fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>>;
}
// InterfaceWriteanager
