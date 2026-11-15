use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("ROM is too large {0} of {1}")]
    RomTooLarge(usize, usize),
    #[error("Address {0:#06X} is out of bounds")]
    AddressOutOfBounds(u16),
    #[error("Opcode {0:#06X} does not exist")]
    OpCodeDoesNotExist(u16),
    #[error("Stackoverflow")]
    StackOverflow,
    #[error("Stackunderflow")]
    StackUnderflow
}