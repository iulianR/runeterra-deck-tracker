#[derive(Debug)]
pub enum Error {}

//impl fmt::Display for Error {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        match *self {
//            Error::Io(ref err) => err.fmt(f),
//            CliError::Csv(ref err) => err.fmt(f),
//            CliError::NotFound => write!(f, "No matching cities with a \
//                                             population were found."),
//        }
//    }
//}

//pub struct Error {
//    inner: Box<Inner>,
//}
//
//struct Inner {
//    kind: Kind,
//    source: Option<Box<dyn std::error::Error>>
//}
//
//impl Error {
//    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
//    where
//        E: Into<dyn std::error::Error>,
//    {
//        Error {
//            inner: Box::new(Inner {
//                kind,
//                source: source.map(Into::into),
//            }),
//        }
//    }
//}
//
//impl std::error::Error for Error {
//    fn source(&self) -> Option<&dyn std::error::Error> {
//        self.inner.source.as_ref().map(|e| &**e as _)
//    }
//}
//
//#[derive(Debug)]
//pub(crate) enum Kind {
//    Io,
//}
