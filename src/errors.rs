use thiserror::Error;

#[derive(Error, Debug)]
pub enum AvlError {
    #[error("key is already taken")]
    KeyOcupied,
}
