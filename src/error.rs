//! Error and result types for runtime error.
/// Handling of runtime errors in main.
#[derive(Debug)]
pub enum Error {
    /// A bit depth that's not in the [range][std::ops::Range] `0..8`
    BadBitDepth(u8),
    /// An error creating a [color::Mode]
    Color(crate::color::Error),
}

/// Result type for [Error]
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BadBitDepth(n) => write!(
                f,
                "configuration error: bit depth must be between 1 and 7, but was {}",
                n
            ),
            Error::Color(err) => write!(f, "configuration error for: {}", err),
        }
    }
}

impl From<crate::color::Error> for Error {
    fn from(err: crate::color::Error) -> Self {
        Error::Color(err)
    }
}

impl std::error::Error for Error {}
