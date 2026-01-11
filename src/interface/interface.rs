/// InterfaceRead that can read from a source and output the source as Result
pub trait InterfaceRead<'input> {
    type Input;
    type Output;
    type OutputError;
    fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError>;
}

// InterfaceWrite
