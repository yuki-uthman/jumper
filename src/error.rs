use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{}", message)]
    GitError { message: String },

    #[error("the result of \"git log {0}\" is empty. maybe wrong branch name?")]
    GitLogEmpty(String),

    #[error("no more changes found for the file: {0}")]
    NoMoreChanges(String),

    #[error("already at the first commit")]
    FirstCommit,

    #[error("already at the last commit")]
    LastCommit,
}
