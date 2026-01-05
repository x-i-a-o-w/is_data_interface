type MultipleError<E = notify::Error> = Vec<E>;
type SingleError = notify::Error;
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("multiple error")]
    Multiple(MultipleError),
    #[error("single error")]
    Single(SingleError),
}

impl From<SingleError> for Error {
    fn from(f: SingleError) -> Self {
        Error::Single(f)
    }
}

impl From<MultipleError> for Error {
    fn from(f: MultipleError) -> Self {
        Error::Multiple(f)
    }
}
