pub type Result<T> = std::result::Result<T, Error>;

type MultipleError = Vec<Error>;
#[derive(thiserror::Error, Debug)]
pub enum Error
// <T = (),E=Box<Error>>
{
    #[error("multiple error")]
    Multiple(MultipleError),
    #[error("Notify error")]
    Notify(#[from] notify::Error),
    #[error("Notify error")]
    Io(#[from] std::io::Error),

    #[error("Any error")]
    Any(#[from] anyhow::Error),
    // #[error("VecResult error")]
    // VecResult(Box<VecResult<T,E>>),
}
impl From<MultipleError> for Error {
    fn from(from: MultipleError) -> Error {
        Error::Multiple(from)
    }
}
// struct InterfaceError{kind:ErrorKind,};enum ErrorKind{	}

pub enum OrResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> OrResult<T, E> {
    pub fn wrap<WrappedOk, WrappedErr, F, G>(
        self,
        ok_wrapper: F,
        err_wrapper: G,
    ) -> OrResult<WrappedOk, WrappedErr>
    where
        F: FnOnce(T) -> WrappedOk,
        G: FnOnce(E) -> WrappedErr,
    {
        match self {
            OrResult::Ok(value) => OrResult::Ok(ok_wrapper(value)),
            OrResult::Err(error) => OrResult::Err(err_wrapper(error)),
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for OrResult<T, E> {
    fn from(from: std::result::Result<T, E>) -> OrResult<T, E> {
        match from {
            Ok(o) => OrResult::Ok(o),
            Err(o) => OrResult::Err(o),
        }
    }
}
impl<T, E> From<OrResult<T, E>> for std::result::Result<T, E> {
    fn from(from: OrResult<T, E>) -> std::result::Result<T, E> {
        match from {
            OrResult::Ok(o) => Ok(o),
            OrResult::Err(o) => Err(o),
        }
    }
}

#[derive(Default, Debug)]
pub struct VecResult<T = (), E = Error> {
    pub ok: T,
    pub err: E,
}

/// this type of error serves as a recoverable error for many operations(vec)
impl<T, E> VecResult<T, E>
where
    T: Default,
    E: Default,
{
    pub fn new() -> VecResult<T, E> {
        VecResult::default()
    }
}

impl<T, E> VecResult<T, E> {
    pub fn push_result(v: &mut VecResult<Vec<T>, Vec<E>>, r: std::result::Result<T, E>) {
        match r {
            Ok(o) => v.ok.push(o),
            Err(e) => v.err.push(e),
        }
    }
    pub fn push_map_ok_result<U, F>(
        v: &mut VecResult<Vec<T>, Vec<E>>,
        r: std::result::Result<U, E>,
        f: F,
    ) where
        F: FnOnce(U) -> T,
    {
        match r {
            Ok(o) => v.ok.push(f(o)),
            Err(e) => v.err.push(e),
        }
    }
    pub fn push_map_err_result<U, F>(
        v: &mut VecResult<Vec<T>, Vec<E>>,
        r: std::result::Result<T, U>,
        f: F,
    ) where
        F: FnOnce(U) -> E,
    {
        match r {
            Ok(o) => v.ok.push(o),
            Err(e) => v.err.push(f(e)),
        }
    }
    pub fn from_split_fill<I, F>(i: I, mut f: F) -> VecResult<Vec<T>, Vec<E>>
    where
        F: FnMut(&I::Item) -> std::result::Result<T, E>,
        I: Iterator,
    {
        let mut ok = Vec::new();
        let mut err = Vec::new();
        i.for_each(|x| match f(&x) {
            Ok(e) => ok.push(e),
            Err(e) => err.push(e),
        });
        VecResult { ok, err }
    }
    pub fn from_error_fill_map<I, F, U>(u: U, i: I, mut f: F) -> VecResult<U, Vec<E>>
    where
        F: FnMut(&I::Item) -> std::result::Result<T, E>,
        I: Iterator,
    {
        let mut err = Vec::new();
        i.for_each(|x| {
            if let Err(e) = f(&x) {
                err.push(e);
            }
        });
        VecResult { ok: u, err }
    }
    pub fn from_error_fill<I, F>(i: I, mut f: F) -> VecResult<(), Vec<E>>
    where
        F: FnMut(&I::Item) -> std::result::Result<T, E>,
        I: Iterator,
    {
        let mut err = Vec::new();
        i.for_each(|x| {
            if let Err(e) = f(&x) {
                err.push(e);
            }
        });
        VecResult { ok: (), err }
    }
    pub fn map<U, C, F>(self, f: F) -> VecResult<U, C>
    where
        F: FnOnce(T, E) -> (U, C),
    {
        let VecResult { ok, err } = self;
        VecResult::from(f(ok, err))
    }
}
impl<T, E> From<(T, E)> for VecResult<T, E> {
    fn from(f: (T, E)) -> VecResult<T, E> {
        VecResult { ok: f.0, err: f.1 }
    }
}
impl<T, E> AsMut<VecResult<T, E>> for VecResult<T, E> {
    fn as_mut(&mut self) -> &mut VecResult<T, E> {
        self
    }
}
impl<T, E> AsRef<VecResult<T, E>> for VecResult<T, E> {
    fn as_ref(&self) -> &VecResult<T, E> {
        self
    }
}
