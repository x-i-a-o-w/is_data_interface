pub mod error;
pub mod fs;
pub mod interface;
pub mod lock;

pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Init;
pub struct Open;
#[derive(Clone)]
pub struct DataInterface<I: interface::RigidInterface, State = Init> {
    #[allow(dead_code)]
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
