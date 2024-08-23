use core::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Failed to initialize devices on the I²C bus
    InitializeError,
    /// Invalid input data provided
    InvalidInputData,
}

// Implement Display for Error<E> if E also implements Display
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InitializeError => write!(f, " Failed to initialize devices on the I²C bus"),
            Error::InvalidInputData => write!(f, "Invalid input data provided"),
        }
    }
}
