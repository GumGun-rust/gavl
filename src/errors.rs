//! test doc
use thiserror::Error;


/// Enum for the possible errors that can be returned by the library
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Occurs when key is taken
    #[error("key is already taken")]
    KeyOcupied, 
    /// Occurs when the key couldn't be found
    #[error("key wasn't found")]
    NotFound,
}
