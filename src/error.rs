use thiserror::Error;



#[derive(Error, Debug)]
pub enum Error {

}

#[derive(Error, Debug)]
pub enum MbcError {
    #[error("Read error: {0}")]
    ReadError(#[from] MbcReadError),
    #[error("Write error: {0}")]
    WriteError(#[from] MbcWriteError),
}

#[derive(Error, Debug)]
pub enum MbcReadError {
    #[error("Read address out of range: {0}")]
    ReadAddressOutOfRange(u32),
}

#[derive(Error, Debug)]
pub enum MbcWriteError {
    #[error("Write address out of range: {0}")]
    WriteAddressOutOfRange(u32),
}