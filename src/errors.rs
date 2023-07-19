//! test doc
use thiserror::Error;


/// hola 
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Occurs when key is taken
    #[error("key is already taken")]
    KeyOcupied, 
    #[error("key wasn't found")]
    NotFound,
}
