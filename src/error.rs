use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("the result of \"git log {0}\" is empty. maybe wrong branch name?")]
    GitLogEmpty(String),
}
