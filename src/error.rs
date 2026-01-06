type NotifyError = notify::Error;
#[derive(thiserror::Error, Debug)]
pub enum Error<T = ()> {
    // #[error("multiple error")]
    // Multiple(MultipleError),
    #[error("Notify error")]
    Notify(#[from] NotifyError),
    #[error("Notify error")]
    Io(#[from] std::io::Error),
    #[error("VecResult error")]
    VecResult(Box<VecResult<T>>),
}

// struct InterfaceError{kind:ErrorKind,};enum ErrorKind{	}

#[derive(Default, Debug)]
pub struct VecResult<T = (), E = Error> {
    pub ok: T,
    pub err: E,
}

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
