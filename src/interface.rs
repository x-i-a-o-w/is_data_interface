// TODO make variants of fns for non anyhow result
/// interface that can read from a source and output the source as Result
pub trait Interface {
    type Output;
    fn read(&self) -> anyhow::Result<Self::Output>;
}
/// interface has the ability to fix the interface if read returns Err(e)
pub trait RigidInterface: Interface {
    fn read(&self) -> anyhow::Result<Self::Output> {
        Interface::read(self)
    }
    fn write(&self) -> anyhow::Result<()>;
}
