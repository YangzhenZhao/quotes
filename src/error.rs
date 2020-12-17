#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Isahc(isahc::Error),
    Xlsx(calamine::XlsxError),
    Calamine(calamine::Error),
}

macro_rules! from_err {
    ($from:ty, $var:tt) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Error {
                Error::$var(e)
            }
        }
    };
}

from_err!(isahc::Error, Isahc);
from_err!(calamine::XlsxError, Xlsx);
from_err!(std::io::Error, Io);
from_err!(calamine::Error, Calamine);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Isahc(e) => write!(f, "Isahc error: {}", e),
            Error::Xlsx(e) => write!(f, "XlsError: {}", e),
            Error::Calamine(e) => write!(f, "Calamine: {}", e),
        }
    }
}
