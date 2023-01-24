use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("the result of git log is empty. maybe wrong branch name?")]
    GitLogEmpty,
}
